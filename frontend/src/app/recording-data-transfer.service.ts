import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

type Recording = {
    name: string;
}

@Injectable({ providedIn: 'root' })
export class RecordingDataTransferService {

    private readonly endpointUrl = '/example';

    constructor(
        private readonly http: HttpClient
    ) {}


    getSimilarRecordings(payload: string, isBinary: boolean): Observable<Recording> {
        return this.http.post<Recording>(this.endpointUrl, {
            payload,
            is_binary: isBinary
        });
    }
}
