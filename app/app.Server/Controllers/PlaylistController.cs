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
    public class PlaylistController : Controller
    {
        [HttpGet("[action]"), Authorize]
        public async Task<List<Playlist>> GetPlaylists()
        {
            var currentUser = HttpContext.User;
            var email = currentUser.FindFirstValue(ClaimTypes.Email);
            using(var client = new HttpClient())
            {
                email = WebUtility.UrlEncode(email);
                var json = await client.GetStringAsync($"http://data/playlists?email={email}");
                return JsonConvert.DeserializeObject<List<Playlist>>(json);
            }
        }

        [HttpGet("[action]"), Authorize]
        public async Task<Playlist> AddPlaylist([FromQuery] string name)
        {
            var currentUser = HttpContext.User;
            var email = currentUser.FindFirstValue(ClaimTypes.Email);
            var parameters = new Dictionary<string, string>()
                {
                    {"email", email},
                    {"name", name},
                };
            using(var client = new HttpClient())
            {
                var response = await client.PostAsync("http://data/playlist", new FormUrlEncodedContent(parameters));
                if(response.IsSuccessStatusCode)
                {
                    var json = await response.Content.ReadAsStringAsync();
                    return JsonConvert.DeserializeObject<Playlist>(json);
                }
                return null;
            }
        }

        [HttpGet("[action]"), Authorize]
        public async Task DeletePlaylist([FromQuery] string token)
        {
            var currentUser = HttpContext.User;
            var email = currentUser.FindFirstValue(ClaimTypes.Email);
            using(var client = new HttpClient())
            {
                email = WebUtility.UrlEncode(email);
                token = WebUtility.UrlEncode(token);
                await client.DeleteAsync($"http://data/playlist?email={email}&token={token}");
            }
        }

        [HttpGet("[action]"), Authorize]
        public async Task<List<Track>> GetTracks([FromQuery] string token)
        {
            var currentUser = HttpContext.User;
            var email = currentUser.FindFirstValue(ClaimTypes.Email);
            using(var client = new HttpClient())
            {
                email = WebUtility.UrlEncode(email);
                token = WebUtility.UrlEncode(token);
                var json = await client.GetStringAsync($"http://data/playlist/tracks?email={email}&token={token}");
                return await FetchDataUtility.Fetch(JsonConvert.DeserializeObject<List<Track>>(json));
            }
        }

        [HttpGet("[action]"), Authorize]
        public async Task AddTracks([FromQuery] string token, [FromQuery] string tracks)
        {
            var currentUser = HttpContext.User;
            var email = currentUser.FindFirstValue(ClaimTypes.Email);
            var parameters = new Dictionary<string, string>()
                {
                    {"email", email},
                    {"token", token},
                    {"tracks", tracks},
                };
            using(var client = new HttpClient())
            {
                await client.PostAsync("http://data/playlist/tracks", new FormUrlEncodedContent(parameters));
            }
        }

        [HttpGet("[action]"), Authorize]
        public async Task DeleteTracks([FromQuery] string token, [FromQuery] string ranks)
        {
            var currentUser = HttpContext.User;
            var email = currentUser.FindFirstValue(ClaimTypes.Email);
            using(var client = new HttpClient())
            {
                email = WebUtility.UrlEncode(email);
                token = WebUtility.UrlEncode(token);
                ranks = WebUtility.UrlEncode(ranks);
                await client.DeleteAsync($"http://data/playlist/tracks?email={email}&token={token}&ranks={ranks}");
            }
        }
    }
}