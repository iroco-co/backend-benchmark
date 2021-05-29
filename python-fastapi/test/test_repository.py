import asynctest
from aiopg import create_pool

from server.repository import Repository, Contact


class TestRepository(asynctest.TestCase):
    def setUp(self):
        self.aio_engine = self.loop.run_until_complete(
            create_pool(user='test', database='test', host='postgresql', password='test'))
        self.repository = Repository(self.aio_engine)

    async def tearDown(self) -> None:
        async with self.aio_engine.acquire() as con:
            cur = await con.cursor()
            await cur.execute('delete from contact')
        self.aio_engine.terminate()
        await self.aio_engine.wait_closed()

    async def test_save_get(self):
        self.assertEqual(1, await self.repository.save(Contact(12, 'firstname', 'lastname', '0123456789', 'e@mail.com')))
        contact = await self.repository.get(12)
        self.assertIsNotNone(contact)
        self.assertEqual(contact.firstname, 'firstname')
        self.assertEqual(contact.lastname, 'lastname')
        self.assertEqual(contact.phone, '0123456789')
        self.assertEqual(contact.email, 'e@mail.com')

    async def test_save_get_all(self):
        self.assertEqual(1, await self.repository.save(Contact(11, 'firstname', 'lastname', '0123456789', 'e@mail.com')))
        self.assertEqual(1, await self.repository.save(Contact(12, 'firstname', 'lastname', '0123456789', 'e@mail.com')))
        self.assertEqual(len(await self.repository.get_all()), 2)
