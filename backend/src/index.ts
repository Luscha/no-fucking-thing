import express from 'express'
import collections from './routes/collections';
import cors from 'cors';

const app = express()
app.use(express.json())
app.use(cors())

app.use('/collections', collections);

app.listen(3000, () =>
  console.log(`
🚀 Server ready at: http://localhost:3000
⭐️ See sample requests: http://pris.ly/e/ts/rest-express#3-using-the-rest-api`),
)