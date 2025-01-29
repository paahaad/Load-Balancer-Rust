import express from "express"

const backend_1 = express()
const backend_2 = express();

let PORT_1=3000
let PORT_2=3001

backend_1.get("/", async (_, res) => res.end("Hello world from backend_1"))
backend_2.get("/", async (_, res) => res.end("Hello world from backedn_2"))

backend_1.listen(PORT_1, () => console.log(`Server stated at ${PORT_1}`))
backend_2.listen(PORT_2, () => console.log(`Server started at ${PORT_2}`))