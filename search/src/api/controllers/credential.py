# -*- coding: utf-8 -*-
import re
from urllib.request import urlopen
from bs4 import BeautifulSoup

URL = 'https://soundcloud.com/altescy/test'

def get_url(url):
    html = urlopen(url).read()
    return html.decode('utf-8')

def find_script_url(html):
    dom = BeautifulSoup(html, 'html.parser')
    scripts = dom.findAll('script', attrs={'src': True})
    for script in scripts:
        src = script['src']
        if 'app' in src.split('/')[-1]:
            return src
    raise RuntimeError('SoundCloud Credential Error: Not Found Script')

def find_client_id(script_text):
    return re.findall(r'client_id:"([a-zA-Z0-9]+)"', script_text)[0]

def get_credential():
    html = get_url(URL)
    script_url = find_script_url(html)
    script = get_url(script_url)
    client_id = find_client_id(script)
    return client_id


if __name__ == '__main__':
    client_id = get_credential()
    print(client_id)