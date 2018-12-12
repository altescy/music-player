using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Text;

namespace app.Shared.Models
{
    public class Playlist
    {
        public string Token { get; set; }
        public string Name { get; set; }
        public List<Track> Tracks { get;  set; }
    }
}