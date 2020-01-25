import { Component, OnDestroy } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { AudioRecordingService } from './audio-recording.service';
import { DomSanitizer, SafeUrl } from '@angular/platform-browser';
import { RecordingDataTransferService } from './recording-data-transfer.service';
import { tap } from 'rxjs/operators';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})

export class AppComponent implements OnDestroy {

  searchQuery = '';
  isRecording = false;
  recordedTime: string;
  blobUrl: SafeUrl;
  isLoadingData = false;
  attemptsCounter;
  foundSongBlob = '';

  private readonly apiUrl = '/example';

  constructor(
    private audioRecordingService: AudioRecordingService,
    private http: HttpClient,
    private sanitizer: DomSanitizer,
    private recordingDataTransferService: RecordingDataTransferService
    ) {

    this.audioRecordingService.recordingFailed().subscribe(() => {
      this.isRecording = false;
    });

    this.audioRecordingService.getRecordedTime().subscribe((time) => {
      this.recordedTime = time;
    });

    this.audioRecordingService.getRecordedBlob().subscribe((data) => {
      this.blobUrl = this.sanitizer.bypassSecurityTrustUrl(URL.createObjectURL(data.blob));
    });
  }

  startRecording() {
    if (!this.isRecording) {
      this.isRecording = true;
      this.audioRecordingService.startRecording();
    }
  }

  abortRecording() {
    if (this.isRecording) {
      this.isRecording = false;
      this.audioRecordingService.abortRecording();
    }
  }

  stopRecording() {
    if (this.isRecording) {
      this.audioRecordingService.stopRecording();
      this.isRecording = false;
    }
  }

  clearRecordedData() {
    this.blobUrl = null;
  }

  ngOnDestroy(): void {
    this.abortRecording();
  }

  submitText(): void {
    this.recordingDataTransferService.getSimilarRecordings(this.searchQuery, false).pipe(
      tap(() => this.isLoadingData = true)
    ).subscribe((data) => {
      this.foundSongBlob = (data as any).blob;
      this.isLoadingData = false;
    });
  }

  submitSound(): void {
    this.recordingDataTransferService.getSimilarRecordings(this.blobUrl.toString(), true).pipe(
      tap(() => this.isLoadingData = true)
    ).subscribe((data) => {
      this.foundSongBlob = (data as any).blob;
      this.isLoadingData = false;
    });
  }

}
