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
    public class AuthController : Controller
    {
        [HttpGet("[action]"), Authorize]
        public User UserInfo()
        {
            var currentUser = HttpContext.User;
            return new User { Email = currentUser.FindFirstValue(ClaimTypes.Email) };
        }
    }
}
