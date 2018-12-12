using app.Shared.Models;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
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
    [Route("api/[controller]")]
    public class LikesController : Controller
    {
        [HttpGet("[action]"), Authorize]
        public async Task<List<Track>> GetTracks()
        {
            var currentUser = HttpContext.User;
            var email = currentUser.FindFirstValue(ClaimTypes.Email);
            using(var client = new HttpClient())
            {
                email = WebUtility.UrlEncode(email);
                var json = await client.GetStringAsync($"http://data/likes?email={email}");
                var tracks = await FetchDataUtility.Fetch(JsonConvert.DeserializeObject<List<Track>>(json));
                tracks.ForEach(t => t.IsLiked = true);
                return tracks;
            }
        }

        [HttpGet("[action]"), Authorize]
        public async Task AddTracks([FromQuery] string tracks)
        {
            var currentUser = HttpContext.User;
            var email = currentUser.FindFirstValue(ClaimTypes.Email);
            var parameters = new Dictionary<string, string>()
                {
                    {"email", email},
                    {"tracks", tracks}
                };
            using(var client = new HttpClient())
            {
                await client.PostAsync("http://data/likes", new FormUrlEncodedContent(parameters));
            }
        }

        [HttpGet("[action]"), Authorize]
        public async Task DeleteTracks([FromQuery] string tracks)
        {
            var currentUser = HttpContext.User;
            var email = currentUser.FindFirstValue(ClaimTypes.Email);
            using(var client = new HttpClient())
            {
                email = WebUtility.UrlEncode(email);
                tracks = WebUtility.UrlEncode(tracks);
                await client.DeleteAsync($"http://data/likes?email={email}&tracks={tracks}");
            }
        }
    }
}