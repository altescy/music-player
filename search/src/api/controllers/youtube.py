# -*- coding: utf-8 -*-

import isodate
import os
from apiclient.discovery import build
from apiclient.errors import HttpError
from oauth2client.tools import argparser
from models.track import Track
from controllers import MAX_RESULT

DEVELOPER_KEY = os.environ['YOUTUBE_API_KEY']
YOUTUBE_API_SERVICE_NAME = "youtube"
YOUTUBE_API_VERSION = "v3"


class YoutubeController:
    PROVIDER = 'youtube'

    def __init__(self):
        self.client = build(
            YOUTUBE_API_SERVICE_NAME,
            YOUTUBE_API_VERSION,
            developerKey=DEVELOPER_KEY
        )

    def _search(self, q, pageToken=None):
        result = self.client.search().list(
            part='id',
            q=q,
            maxResults=MAX_RESULT,
            pageToken=pageToken,
        ).execute()
        ids = [item['id'].get('videoId') for item in result.get('items', []) if item['id'].get('videoId')]
        nextPageToken = result.get('nextPageToken')
        return ids, nextPageToken

    def fetch(self, ids):
        assert type(ids) is list
        if not ids: return []
        result = self.client.videos().list(
            part='id,snippet,contentDetails,statistics',
            id=','.join(ids)
        ).execute()
        tracks = []
        for item in result.get('items', []):
            try:
                track = Track(
                    provider=YoutubeController.PROVIDER,
                    contentId=item['id'],
                    accountId=item['snippet']['channelId'],
                    accountName=item['snippet']['channelTitle'],
                    title=item['snippet']['title'],
                    description=item['snippet']['description'],
                    playbackCount=int(item['statistics']['viewCount']),
                    likesCount=int(item['statistics']['likeCount']),
                    duration=isodate.parse_duration(item['contentDetails']['duration']).seconds*1000,
                    thumbnailSmall=item['snippet']['thumbnails'].get('default', {'url':''})['url'],
                    thumbnailMedium=item['snippet']['thumbnails'].get('medium', {'url':''})['url'],
                    thumbnailLarge=item['snippet']['thumbnails'].get('standard', {'url':''})['url'],)
            except (KeyError, ValueError) as e:
                print(e)
                continue
            tracks.append(track)
        return tracks

    def search(self, q, page=None):
        ids, nextPage = self._search(q, page)
        tracks = self.fetch(ids)
        return tracks, nextPage
