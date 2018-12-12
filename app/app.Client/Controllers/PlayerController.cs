using Microsoft.AspNetCore.Blazor;
using Microsoft.JSInterop;
using System.Threading.Tasks;
using app.Shared.Models;

namespace app.Client
{
    public static class PlayerController
    {
        public static Task<bool> Setup()
        {
            return JSRuntime.Current.InvokeAsync<bool>("player_controls.setup");
        }
        public static Task Load(Track track)
        {
            return JSRuntime.Current.InvokeAsync<object>(
                "player_controls.load", track.Provider, track.ContentId);
        }
    }
}