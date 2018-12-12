using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Text;

namespace app.Shared.Models
{
    public class Track
    {
        public string Provider { get; set; }
        public string ContentId { get; set; }
        public string AccountId { get; set; }
        public string AccountName { get; set; }
        public string Title { get; set; }
        public string Description { get; set; }
        public int PlaybackCount { get; set; }
        public int LikesCount { get; set; }
        public int Duration { get; set; }
        public string ThumbnailSmall { get; set; }
        public string ThumbnailMedium { get; set; }
        public string ThumbnailLarge { get; set; }
        public int rankorder { get; set; } = -1;
        public bool IsLiked { get; set; } = false;
        public bool NowPlaying { get; set; } = false;
    }
}