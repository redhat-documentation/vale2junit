# vale2junit

Convert the JSON output from [Vale](https://vale.sh/) to the [JUnit](https://junit.org/) format.

This tool is intended as a CI/CD component that you use to turn Vale output into JUnit, which is a format that [GitLab can present](https://docs.gitlab.com/ee/ci/testing/unit_test_reports.html) as human-readable test results in its pipeline.


## Installation

This tool is available as a container at <https://quay.io/repository/redhat-documentation/vale2junit>. If you want to install it locally, use Docker or Podman:

1. Download the image:

    ```
    $ docker pull quay.io/redhat-documentation/vale2junit:latest
    ```

2. Configure a command alias. Save this line in your shell configuration file, such as in the `~/.bashrc` file:

    ```bash
    alias vale2junit="docker run -it -v .:/mnt/vale2junit:Z redhat-documentation/vale2junit vale2junit"
    ```

3. Open a new terminal to reload the shell configuration.

4. Test that `vale2junit` works:

    ```
    ~/directory]$ vale2junit
    ```

    Because of SELinux permissions, you cannot run this command at the root of your home directory. Run it in a subdirectory.


## Local usage

1. Install Vale. See <https://vale.sh/>.

2. Configure Vale in the `~/.vale.ini` file. For example:

    ```ini
    StylesPath = styles
    
    MinAlertLevel = warning
    
    Packages = RedHat
    
    [*]
    
    BasedOnStyles = RedHat
    ```

3. Apply the Vale configuration:

    ```
    $ vale sync
    ```

4. Test file with Vale and save the output:

    ```
    $ vale --output JSON file1.adoc file2.adoc > vale-report.json
    ```

5. Convert the Vale results to JUnit:

    ```
    $ vale2junit --file=vale-report.json --out=report.xml
    ```

    Optionally, you can also generate a CSV output in addition to JUnit:

    ```
    $ vale2junit --file=vale-report.json --out=report.xml --csv=report.csv
    ```

6. The `report.xml` file contains the JUnit output.


## CI/CD usage

In CI/CD, you can deploy the tool in various ways depending on your Git platform, your documentation file layout, and other factors.

The following is an example of a GitLab CI/CD configuration in a repository of AsciiDoc documentation files.

It requires that you set the `VALE_CI_API_TOKEN` variable to upload test results to GitLab. See [Create a deploy token](https://docs.gitlab.com/ee/user/project/deploy_tokens/index.html#create-a-deploy-token).

### `.gitlab-ci.yml`

```yaml
# Run this whole file only in merge requests.
workflow:
  rules:
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
      when: never
    - if: $CI_PIPELINE_SOURCE == 'merge_request_event'

# Prepare some common resources for all future tests.
shared_setup:
  stage: .pre
  image: bitnami/git
  tags: [shared]
  script:
    - mkdir _setup
    - |
      changed_files=$(git diff --name-only \
          origin/${CI_MERGE_REQUEST_TARGET_BRANCH_NAME}...origin/${CI_MERGE_REQUEST_SOURCE_BRANCH_NAME} \
          -- '*\.adoc')

    - |
      for file in ${changed_files}; do
           if test -f ${file}; then
               echo -n "${file} " >> _setup/changed-files.txt
           fi
      done

    - |
      if ! [ -s _setup/changed-files.txt ]; then
          echo "No changed files detected"
          exit 0
      fi

    - git checkout origin/master -- .gitlab-ci.yml

  artifacts:
    paths: ['_setup']

# Perform tests with Vale
vale_test:
  image:
    name: jdkato/vale
    entrypoint: [""]
  tags: [shared]
  stage: test

  before_script:
    - |
      if ! [ -s _setup/changed-files.txt ]; then
          echo "No changed files detected"
          exit 0
      fi

  script:
    - echo -e "StylesPath = styles\n\nMinAlertLevel = warning\n\nPackages = RedHat\n\n[*]\n\nBasedOnStyles = RedHat" > .vale.ini
    - vale sync
    - cat _setup/changed-files.txt | xargs vale --no-exit --output JSON > _setup/vale-report.json

  artifacts:
    paths: ['_setup']


# Convert the Vale test results to JUnit
vale_test_results:
  image: quay.io/redhat-documentation/vale2junit:latest
  tags: [shared]
  stage: .post
  variables:
    AUTH_HEADER: 'Authorization: Bearer ${VALE_CI_API_TOKEN}'
    POST_URL: '${CI_API_V4_URL}/projects/${CI_MERGE_REQUEST_PROJECT_ID}/merge_requests/${CI_MERGE_REQUEST_IID}/notes'
    SUCCESS_JSON: '{ "body": ":white_check_mark: All files passed Vale validation." }'
    FAIL_JSON: '{ "body": ":x: Vale validation failed. Go to [test reports](${CI_PROJECT_URL}/-/pipelines/${CI_PIPELINE_ID}/test_report) and click vale_test_results to see the itemized list of failures." }'

  before_script:
    - |
      if ! [ -s _setup/changed-files.txt ]; then
          echo "No changed files detected"
          exit 0
      fi

  script:
    - vale2junit --version
    - vale2junit --verbose --file=_setup/vale-report.json --out=report.xml --csv=report.csv

  after_script:
    - |
      if [ -s _setup/changed-files.txt ]; then
          echo "Changed files detected"
          if grep 'failures="[123456789]' report.xml >/dev/null ; then
              curl -k --request POST --header "$AUTH_HEADER" --header "Content-Type: application/json" "$POST_URL" --data-raw "$FAIL_JSON"
          else
              curl -k --request POST --header "$AUTH_HEADER" --header "Content-Type: application/json" "$POST_URL" --data-raw "$SUCCESS_JSON"
          fi
      else
          echo "No changed files detected" && exit 0
      fi

  artifacts:
    when: always
    paths:
      - "report.csv"
    reports:
      junit: report.xml
    expire_in: 1 week
```
