export const lastFM = {
    track: {
        getInfo: async ({ track, artist, username, autocorrect }: { track: string, artist: string, username?: string, autocorrect?: boolean }) => {
            let url = `https://ws.audioscrobbler.com/2.0/?method=track.getInfo&api_key=${import.meta.env.VITE_LASTFM_API_KEY}&track=${track}&artist=${artist}&format=json`
            let response = await fetch(url);
            let data = await response.json();
            return data;
        }
    },
};