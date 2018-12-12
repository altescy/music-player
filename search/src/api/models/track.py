# -*- coding: utf-8 -*-

import dataclasses


@dataclasses.dataclass
class Track:
    provider: str
    contentId: str
    accountId: str
    accountName: str
    title: str
    description: str
    playbackCount: int
    likesCount: int
    duration: int
    thumbnailSmall: str
    thumbnailMedium: str
    thumbnailLarge: str
