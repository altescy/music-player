using System;
using System.Net;

namespace app.Client.Utilities
{
    public static class UrlManager
    {
        public static string UrlEncode(string s)
        {
            return WebUtility.UrlEncode(s);
        }
    }
}