import express from 'express'
import { Prisma, PrismaClient } from '@/generated/client'
import { signedMessageOK } from '@/middlewares/signed-message'
import { CW721OK } from '@/middlewares/cw721'
import { CW721Info } from '@/contract/cw721'

const prisma = new PrismaClient()

const router = express.Router();

router.post(`/`, [signedMessageOK(), CW721OK()], async (req, res) => {
    const { contractAddrs } = req.body.message
    const info = await CW721Info(contractAddrs)

    const result = await prisma.collection.create({
        data: {
            contractAddress: contractAddrs,
            name: info.name,
            minter: info.minter
        },
    })
    res.json(result)
})

router.get('/', async (req, res) => {
    const { searchString, skip, take, orderBy } = req.query
    const name: Prisma.CollectionWhereInput = searchString ? 
        { name : { contains: searchString as string } } : {}

    const collections = await prisma.collection.findMany({
        where: name,
        take: Number(take) || undefined,
        skip: Number(skip) || undefined,
        orderBy: {
            name: orderBy as Prisma.SortOrder,
        },
    })

    res.json(collections)
})


// router.get('/feed', async (req, res) => {
//     const { searchString, skip, take, orderBy } = req.query

//     const or: Prisma.PostWhereInput = searchString
//         ? {
//             OR: [
//                 { title: { contains: searchString as string } },
//                 { content: { contains: searchString as string } },
//             ],
//         }
//         : {}

//     const posts = await prisma.post.findMany({
//         where: {
//             published: true,
//             ...or,
//         },
//         include: { author: true },
//         take: Number(take) || undefined,
//         skip: Number(skip) || undefined,
//         orderBy: {
//             updatedAt: orderBy as Prisma.SortOrder,
//         },
//     })

//     res.json(posts)
// })

export default router;