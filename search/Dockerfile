FROM python:3.7-alpine

WORKDIR /app
COPY . ./

RUN pip install -r requirements.txt

EXPOSE 80

CMD ["python", "src/api/main.py"]