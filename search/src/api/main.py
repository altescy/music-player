# -*- coding: utf-8 -*-
import argparse
import dataclasses
from flask import Flask, request, abort, jsonify
from urllib.parse import quote
from urllib.error import HTTPError

from controllers.youtube import YoutubeController
from controllers.soundcloud import SoundcloudController
from utils import parse_search_query, parse_fetch_query

api = Flask(__name__)

PROVIDERS = ['youtube', 'soundcloud']

controllers = {
    'youtube': YoutubeController(),
    'soundcloud': SoundcloudController(),
}

def search_provider(provider, request):
    if provider not in PROVIDERS: abort(400)
    query = parse_search_query(request)
    if not query: abort(400)
    try:
        tracks, nextPage = controllers[provider].search(**query)
    except HTTPError as e:
        print(e)
        abort(503)
    tracks = [dataclasses.asdict(track) for track in tracks]
    next_uri = None
    if nextPage:
        next_uri = '{}?q={}&page={}'.format(quote(provider), quote(query['q']), quote(nextPage))
    response = {
        'query': str(query),
        'result': {
            'tracks': tracks,
            'nextUri': next_uri
        },
    }
    return response

def fetch_provider(provider, request):
    if provider not in PROVIDERS: abort(400)
    query = parse_fetch_query(request)
    if not query: abort(400)
    try:
        tracks = controllers[provider].fetch(**query)
    except HTTPError as e:
        print(e)
        abort(503)
    tracks = [dataclasses.asdict(track) for track in tracks]
    response = {
        'query': str(query),
        'result': {
            'tracks': tracks,
        },
    }
    return response

@api.route('/', methods=['GET'])
def index():
    response = {
        "result": "search api",
    }
    return jsonify(response)

@api.route('/search/<provider>', methods=['GET'])
def search(provider):
    response = search_provider(provider, request)
    return jsonify(response)

@api.route('/fetch/<provider>')
def fetch(provider):
    response = fetch_provider(provider, request)
    return jsonify(response)

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--host', default='0.0.0.0')
    parser.add_argument('--port', default=80)
    args = parser.parse_args()

    api.run(host=args.host, port=args.port, threaded=True)
