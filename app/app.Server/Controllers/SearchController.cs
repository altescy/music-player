using app.Shared.Models;
using Microsoft.AspNetCore.Mvc;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Net.Http;
using System.Threading.Tasks;
using Newtonsoft.Json;

namespace app.Server.Controllers
{
    [Route("api/[controller]")]
    public class SearchController : Controller
    {
        [HttpGet("[action]")]
        public async Task<SearchResult> Tracks([FromQuery] string q, [FromQuery] string provider)
        {
            using(var client = new HttpClient())
            {
                q = WebUtility.UrlEncode(q);
                provider = WebUtility.UrlEncode(provider);
                string url = $"http://search/search/{provider}?q={q}";
                var json = await client.GetStringAsync(url);
                var response = JsonConvert.DeserializeObject<SearchResponse>(json);
                return response.Result;
            }
        }

        [HttpGet("[action]")]
        public async Task<SearchResult> Next([FromQuery] string next)
        {
            using(var client = new HttpClient())
            {
                string url = $"http://search/search/{next}";
                var json = await client.GetStringAsync(url);
                var response = JsonConvert.DeserializeObject<SearchResponse>(json);
                return response.Result;
            }
        }
    }
}
