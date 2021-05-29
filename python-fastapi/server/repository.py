from typing import List

from pydantic import BaseModel


class Contact:
    def __init__(self, id:int, firstname: str, lastname: str, phone: str, email: str) -> None:
        self.id = id
        self.email = email
        self.phone = phone
        self.lastname = lastname
        self.firstname = firstname


class Repository:
    def __init__(self, db) -> None:
        self.db = db

    async def get(self, cid: int) -> Contact:
        async with self.db.acquire() as con:
            cur = await con.cursor()
            await cur.execute('SELECT id, firstname, lastname, phone, email from contact where id=%s', (cid,))
            row = await cur.fetchone()
            return Contact(row[0], row[1], row[2], row[3], row[4])

    async def save(self, contact: Contact) -> int:
        async with self.db.acquire() as con:
            cur = await con.cursor()
            await cur.execute("INSERT INTO contact (id, firstname, lastname, phone, email) VALUES (%s, %s, %s, %s, %s)",
                                (contact.id, contact.firstname, contact.lastname, contact.phone, contact.email))
            return cur.rowcount

    async def get_all(self) -> List[Contact]:
        async with self.db.acquire() as con:
            cur = await con.cursor()
            await cur.execute('SELECT id, firstname, lastname, phone, email from contact')
            rows = await cur.fetchall()
            return [Contact(row[0], row[1], row[2], row[3], row[4]) for row in rows]



