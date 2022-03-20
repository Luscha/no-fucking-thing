import express from 'express'
import collections from './routes/collections';
import probes from './routes/probes';
import cors from 'cors';

import { PORT, HOST } from './config';

const app = express()
app.use(express.json())
app.use(cors())

app.use('/collections', collections);
app.use('/', probes);

app.listen({ port: PORT, host: HOST }, () =>
  console.log(`
🚀 Server ready at: ${PORT}:${HOST}
⭐️ See sample requests: http://pris.ly/e/ts/rest-express#3-using-the-rest-api`),
)