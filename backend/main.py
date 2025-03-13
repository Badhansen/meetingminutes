from fastapi import FastAPI, WebSocket
from uvicorn import run

app = FastAPI()


@app.get("/")
async def home():
    return {"message": "FastAPI Transcription Server is Running"}

# Add this to make the script runnable
if __name__ == "__main__":
    run(app, host="127.0.0.1", port=8000)
