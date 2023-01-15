import { Notification } from './messages/notifications';
import { Subject } from 'rxjs';
import { Command } from './messages/commands';

export class WebrtcServer {
  private channel: RTCDataChannel;
  public notifications$: Subject<Notification> = new Subject();

  public sendCommand(command: Command) {
    console.log('sending command ', command.commandType);
    let command_bytes = Command.encode(command).finish();
    this.channel.send(command_bytes);
  }

  start() {
    let SESSION_ADDRESS = '/new_rtc_session';

    let peer = new RTCPeerConnection({
      iceServers: [{
        urls: ['stun:stun.l.google.com:19302'],
      }],
    });

    this.channel = peer.createDataChannel('data', {
      ordered: false,
      maxRetransmits: 0,
    });
    this.channel.binaryType = 'arraybuffer';

    this.channel.onerror = function(evt) {
      console.log('data channel error:', evt.error);
    };

    let last_received_time = Date.now();
    this.channel.onopen = () => {
      console.log('data channel open');

      this.channel.onmessage = (evt) => {
        let notification_bytes = new Uint8Array(evt.data);
        let updateDuration = Date.now() - last_received_time;
        console.log(
          'received data of length ' + evt.data.byteLength,
          'in ' + updateDuration + ' ms',
        );
        if (updateDuration > 200) {
          console.log('----------------------');
        }
        last_received_time = Date.now();

        const notification = Notification.decode(notification_bytes);
        console.log(notification);
        this.notifications$.next(notification);
      };
    };

    peer.onicecandidate = function(evt) {
      if (evt.candidate) {
        console.log('received ice candidate', evt.candidate);
      } else {
        console.log('all local candidates received');
      }
    };

    peer.createOffer().then(function(offer) {
      return peer.setLocalDescription(offer);
    }).then(function() {
      var request = new XMLHttpRequest();
      request.open('POST', SESSION_ADDRESS);
      request.onload = function() {
        if (request.status == 200) {
          var response = JSON.parse(request.responseText);
          peer.setRemoteDescription(new RTCSessionDescription(response.answer)).then(function() {
            var candidate = new RTCIceCandidate(response.candidate);
            peer.addIceCandidate(candidate).then(function() {
              console.log('add ice candidate success');
            }).catch(function(err) {
              console.log('error during \'addIceCandidate\':', err);
            });
          }).catch(function(e) {
            console.log('error during \'setRemoteDescription\':', e);
          });
        }
      };
      request.send(peer.localDescription.sdp);
    }).catch(function(reason) {
      console.log('error during \'createOffer\':', reason);
    });
  }
}
