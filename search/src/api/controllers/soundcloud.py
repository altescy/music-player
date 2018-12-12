# -*- coding: utf-8 -*-

import soundcloud
from controllers.credential import get_credential
from controllers import MAX_RESULT
from models.track import Track


class SoundcloudController:
    PROVIDER = 'soundcloud'
    CLIENT_ID = get_credential()

    @classmethod
    def UPDATE_CREDENTIAL(cls):
        try:
            client_id = get_credential()
        except:
            return
        cls.CLIENT_ID = client_id

    def __init__(self):
        SoundcloudController.UPDATE_CREDENTIAL()
        self.client = soundcloud.Client(client_id=SoundcloudController.CLIENT_ID)

    def parse_result(self, result):
        tracks = []
        for item in result['collection']:
            try:
                track = Track(
                    provider=SoundcloudController.PROVIDER,
                    contentId=str(item['id']),
                    accountId=str(item['user']['id']),
                    accountName=item['user']['username'],
                    title=item['title'],
                    description=item['description'],
                    playbackCount=int(item['playback_count']),
                    likesCount=int(item['likes_count']),
                    duration=int(item['duration']),
                    thumbnailSmall=self.get_thumburl(item['artwork_url'], 'small'),
                    thumbnailMedium=self.get_thumburl(item['artwork_url'], 'medium'),
                    thumbnailLarge=self.get_thumburl(item['artwork_url'], 'large'),)
            except (KeyError, ValueError) as e:
                print(e)
                continue
            tracks.append(track)
        return tracks

    def fetch(self, ids):
        result = self.client.get(
            '/tracks',
            ids=','.join(ids))
        result = {'collection': [item.fields() for item in result]}
        tracks = self.parse_result(result)
        return tracks

    def search(self, q, page=None):
        if(page):
            result = self.client.get(page).fields()
        else:
            result = self.client.get(
                '/tracks',
                q=q,
                linked_partitioning=True,
                limit=MAX_RESULT).fields()
        tracks = self.parse_result(result)
        nextPage = result.get('next_href')
        return tracks, nextPage

    def get_thumburl(self, url, size):
        assert size in ['small', 'medium', 'large']
        try:
            fwd, ext = url.rsplit('.', 1)
            fwd, _ = fwd.rsplit('-', 1)
            size_type = {
                'small': 'large',
                'medium': 't300x300',
                'large': 't500x500'}[size]
            return "{}-{}.{}".format(fwd, size_type, ext)
        except:
            return url
