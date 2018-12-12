using app.Shared.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Net.Http;
using System.Security.Claims;
using System.Threading.Tasks;
using Newtonsoft.Json;

namespace app.Server.Controllers
{
    public static class FetchDataUtility
    {
        public static async Task<List<Track>> Fetch(List<Track> tracks)
        {
            using(var client = new HttpClient())
            {
                var ytIds = from t in tracks where t.Provider == "youtube" select t.ContentId;
                var scIds = from t in tracks where t.Provider == "soundcloud" select t.ContentId;
                var ts = new List<Track>();
                if(ytIds.Count() > 0)
                {
                    var ytJson = await client.GetStringAsync($"http://search/fetch/youtube?ids={WebUtility.UrlEncode(string.Join(",", ytIds))}");
                    ts.AddRange(JsonConvert.DeserializeObject<SearchResponse>(ytJson).Result.Tracks);
                }
                if(scIds.Count() > 0)
                {
                    var scJson = await client.GetStringAsync($"http://search/fetch/soundcloud?ids={WebUtility.UrlEncode(string.Join(",", scIds))}");
                    ts.AddRange(JsonConvert.DeserializeObject<SearchResponse>(scJson).Result.Tracks);
                }
                return tracks.Select(t => {
                    var rankorder = t.rankorder;
                    var s = ts.FirstOrDefault(t_=> t_.Provider == t.Provider && t_.ContentId == t.ContentId);
                    s.rankorder = rankorder;
                    return s;
                }).ToList();
            }
        }
    }
}