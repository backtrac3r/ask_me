import copy
import asyncio
import requests

from fastapi import FastAPI, Request
from pydantic import BaseModel
from llama_cpp import Llama
from sse_starlette import EventSourceResponse

class ModelReq(BaseModel):
    q: str

# load the model
print("Loading model...")
llm = Llama(model_path="ggml-vic13b-q4_0.bin")
print("Model loaded.")

app = FastAPI()

@app.get("/")
async def hello():
    return "FuckMe"

@app.post("/model")
async def model(req: Request, q: ModelReq):
    stream = llm(
        q.q,
        stream=True
    )

    async def async_generator():
        for item in stream:
            yield item
    
    async def server_sent_events():
        async for item in async_generator():
            if await req.is_disconnected():
                break

            result = copy.deepcopy(item)
            text = result["choices"][0]["text"]

            yield {"data": text}

    return EventSourceResponse(server_sent_events())
