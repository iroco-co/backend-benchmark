from aiopg import create_pool
from fastapi import FastAPI

from server.repository import Repository


app = FastAPI()

repository = Repository(None)


@app.on_event("startup")
async def startup_event():
    aio_engine = await create_pool(user='classe', database='classe', host='postgresql', password='classe')
    repository.db = aio_engine


@app.get("/")
async def root():
    return 'contact API'


@app.get("/contacts")
async def contacts():
    return await repository.get_all()


@app.get("/contacts/{contact_id}")
async def contacts(contact_id: int):
    return await repository.get(contact_id)
