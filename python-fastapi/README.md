# python async implementation

to run the server : 

```
pipenv install
pipenv run uvicorn contactserver.main:app --host 0.0.0.0 --no-access-log
```

To build a dist (after having installed the pipenv): 
```
pipenv run python setup.py sdist
```
