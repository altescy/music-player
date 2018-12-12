using app.Shared.Models;
using Microsoft.AspNetCore.Authorization;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Net.Http;
using System.Security.Claims;
using System.Threading.Tasks;
using Newtonsoft.Json;

namespace app.Server.DataAccess
{
    public static class DataIO
    {
        public static async Task<List<Track>> GetTracks(string uri, Dictionary<string, string> parameters)
        {
            using(var client = new HttpClient())
            {
                string qp = "";
                foreach(KeyValuePair<string, string> item in parameters){
                    qp += $"{WebUtility.UrlEncode(item.Key)}={WebUtility.UrlEncode(item.Value)}&";
                }
                var json = await client.GetStringAsync($"http://data/{uri}?{qp}");
                return JsonConvert.DeserializeObject<List<Track>>(json);
            }
        }

        public static async Task Post(string uri, Dictionary<string, string> parameters)
        {
            using(var client = new HttpClient())
            {
                await client.PostAsync($"http://data/{uri}", new FormUrlEncodedContent(parameters));
            }
        }
    }
}