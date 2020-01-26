import { Component, OnDestroy } from '@angular/core';
import { AudioRecordingService } from './audio-recording.service';
import { DomSanitizer, SafeUrl } from '@angular/platform-browser';
import { RecordingDataTransferService, Recording } from './recording-data-transfer.service';
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
  blob: Blob;
  blobSafeUrl: SafeUrl;
  isLoadingData = false;
  attemptsCounter;
  foundSongSafeUrl: null | SafeUrl;
  foundSong: Recording;

  constructor(
      private audioRecordingService: AudioRecordingService,
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
      this.blob = data.blob;
      this.blobSafeUrl = this.sanitizer.bypassSecurityTrustUrl(URL.createObjectURL(data.blob));
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
    this.blob = null;
    this.blobSafeUrl = null;
  }

  ngOnDestroy(): void {
    this.abortRecording();
  }

  submitText(): void {
    this.recordingDataTransferService.getSimilarRecordingByQuery(this.searchQuery).pipe(
      tap(() => this.isLoadingData = true)
    ).subscribe(this.setSongResult);
  }

  submitSound(): void {
    this.recordingDataTransferService.getSimilarRecordingByAudio(this.blob).pipe(
      tap(() => this.isLoadingData = true)
    ).subscribe(this.setSongResult);
  }

  setSongResult = (data: Recording) => {
    this.foundSong = data;
    this.foundSongSafeUrl = this.sanitizer.bypassSecurityTrustUrl(data.url);
    this.isLoadingData = false;
  }

}
