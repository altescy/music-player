using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Text;

namespace app.Shared.Models
{
    public class SearchResponse
    {
        public string Query { get; set; }
        public SearchResult Result { get; set; }
    }

    public class SearchResult
    {
        public List<Track> Tracks { get; set; }
        public string NextUri { get; set; }
    }
}