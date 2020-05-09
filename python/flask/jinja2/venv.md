


- ``Error: Command '['C:\\Users\\13298\\Desktop\\software_course_design\\server\\venv\\Scripts\\python.exe', '-Im', 'ensurepip', '--upgrade', '--default-pip']' returned non-zero exit status 1.``

    - Anaconda没有ensurepip

    - ``python -m venv --without-pip venv``创建没有pip的虚拟环境
    -   ``.\venv\Scripts\activate``
    -   ``.\venv\Scripts\deactivate``


- Error: Could not locate Flask application. You did not provide the FLASK_APP environment variable.
    - powershell: `` $env:FLASK_APP="flasky.py"``
    - cmd: ``set FLASK_APP=flasky.py``