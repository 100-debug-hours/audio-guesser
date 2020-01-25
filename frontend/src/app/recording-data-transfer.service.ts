import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

export type Recording = {
    name: string;
};

@Injectable({ providedIn: 'root' })
export class RecordingDataTransferService {

    private readonly endpointUrl = '/example';

    constructor(
        private readonly http: HttpClient
    ) {}

    getSimilarRecordingByQuery(query: string): Observable<Recording> {
        return this.http.post<Recording>(this.endpointUrl, {
            payload: query
        });
    }

    getSimilarRecordingByAudio(blob: Blob): Observable<Recording> {
        const formData = new FormData();
        formData.append("file", blob, "my_file_name");

        return this.http.post<Recording>(this.endpointUrl, formData);
    }
}
