import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { map, flatMap, tap, catchError } from 'rxjs/operators';

export type Recording = {
    title: string;
    artist: string;
    url: string;
};
type ByQueryRes = {
    status: string,
    result: {
        song_id: string,
        artist_id: string,
        title: string,
        title_with_featured: string,
        full_title: string,
        artist: string,
        lyrics: string
    }[],
};
type ByFileRes = {
    status: string,
    result: {
        list: {
            title: string,
            artist: string,
        }[],
    }
};

@Injectable({ providedIn: 'root' })
export class RecordingDataTransferService {

    constructor(
        private readonly http: HttpClient
    ) {}

    fetch_url_track(author: string, title: string): Observable<string> {
        return this.http.get<DeezerPreview.RootObject>(
            `https://cors-anywhere.herokuapp.com/api.deezer.com/search/?q=${encodeURIComponent(author)
            }&title=${encodeURIComponent(title)}&index=0&limit=1`
        ).pipe(map(res => res.data[0].preview));
    }

    getSimilarRecordingByQuery(query: string): Observable<Recording> {
        return this.http.post<ByQueryRes>("/api/recognize_text", {payload: query }).pipe(
            tap(data => {
                console.dir({ tak_sho_tut_za_govno_query: data });
            }),
            flatMap(
            ({result: [{artist, title}]}) => this
                .fetch_url_track(title, artist)
                .pipe(map(url => ({ artist, title, url }))),
            ),
            catchError(err => {
                alert("Sorry, song was not found! Try again later please! Donates are welcome!");
                throw err;
            })
        );
    }

    getSimilarRecordingByAudio(blob: Blob): Observable<Recording> {
        const formData = new FormData();
        formData.append("file", blob, "my_file_name");

        return this.http.post<ByFileRes>("/api/recognize_file", formData).pipe(
            tap(data => {
                console.dir({ tak_sho_tut_za_govno_music: data });
            }),
            flatMap(
            ({result: { list: [{artist, title}]} }) => this
                .fetch_url_track(title, artist)
                .pipe(map(url => ({ artist, title, url }))),
            ),
            catchError(err => {
                alert("Sorry, song was not found! Try again later please! Donates are welcome!");
                console.error(err);
                throw err;
            })
        );
    }
}


// tslint:disable-next-line: no-namespace
namespace DeezerPreview {

    export interface Artist {
        id: number;
        name: string;
        link: string;
        picture: string;
        picture_small: string;
        picture_medium: string;
        picture_big: string;
        picture_xl: string;
        tracklist: string;
        type: string;
    }

    export interface Album {
        id: number;
        title: string;
        cover: string;
        cover_small: string;
        cover_medium: string;
        cover_big: string;
        cover_xl: string;
        tracklist: string;
        type: string;
    }

    export interface Datum {
        id: number;
        readable: boolean;
        title: string;
        title_short: string;
        title_version: string;
        link: string;
        duration: number;
        rank: number;
        explicit_lyrics: boolean;
        explicit_content_lyrics: number;
        explicit_content_cover: number;
        preview: string;
        artist: Artist;
        album: Album;
        type: string;
    }

    export interface RootObject {
        data: Datum[];
        total: number;
        next: string;
    }

}
