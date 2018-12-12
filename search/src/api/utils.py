# -*- coding: utf-8 -*-

def parse_search_query(request):
    query = request.args.get('q') or request.form['q']
    if not query:
        return None
    page = request.args.get('page')
    return {
        'q': query,
        'page': page,
    }


def parse_fetch_query(request):
    ids = request.args.get('ids') or request.form['ids']
    if not ids:
        return None
    ids = ids.split(',')
    return {'ids': ids}
