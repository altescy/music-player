using app.Shared.Models;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Security.Claims;
using System.Threading.Tasks;
using Newtonsoft.Json;

namespace app.Server.Controllers
{
    [Route("api/[controller]")]
    public class UserController : Controller
    {
        [HttpGet("[action]"), Authorize]
        public async Task<User> AddUser()
        {
            var currentUser = HttpContext.User;
            var email = currentUser.FindFirstValue(ClaimTypes.Email);
            var parameters = new Dictionary<string, string>()
                {
                    {"email", email},
                };
            using(var client = new HttpClient())
            {
                await client.PostAsync("http://data/user", new FormUrlEncodedContent(parameters));
            }
            return new User { Email = email };
        }
    }
}


