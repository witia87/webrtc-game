import { Component } from 'super-ecs';

export class WebrtcSender implements Component {
	public name: symbol = Symbol('WebrtcSender');

  start() {
    let NUM_PACKETS = 1000;
    let SEND_INTERVAL = 100;
    let SESSION_ADDRESS = "/new_rtc_session";

    let peer = new RTCPeerConnection({
      iceServers: [{
        urls: ["stun:stun.l.google.com:19302"]
      }]
    });

    let channel = peer.createDataChannel("data", {
      ordered: false,
      maxRetransmits: 0
    });
    channel.binaryType = "arraybuffer";

    channel.onerror = function(evt) {
      console.log("data channel error:", evt.error);
    };

    let send_times = {};
    let last_received_time = Date.now();
    channel.onopen = function() {
      console.log("data channel open");

      let packet_number = 0
      let cancel = setInterval(function() {
        if (packet_number == NUM_PACKETS) {
          clearInterval(cancel);
          console.log("closing connection");
          peer.close();
        } else {
          packet_number++
          console.log("sending packet", packet_number);
          let array = new Uint8Array(1000);
          array[0] = packet_number;
          channel.send(array);
          send_times[packet_number] = Date.now();
        }
      }, SEND_INTERVAL);

      channel.onmessage = function(evt) {
        let array = new Uint8Array(evt.data);
        let num = Number(array[0]);
        console.log(
          "received packet " + num,
          "of length " + evt.data.byteLength,
          "in " + (Date.now() - last_received_time) + " ms"
        );
        last_received_time = Date.now();
      };
    };

    peer.onicecandidate = function(evt) {
      if (evt.candidate) {
        console.log("received ice candidate", evt.candidate);
      } else {
        console.log("all local candidates received");
      }
    };

    peer.createOffer().then(function(offer) {
      return peer.setLocalDescription(offer);
    }).then(function() {
      var request = new XMLHttpRequest();
      request.open("POST", SESSION_ADDRESS);
      request.onload = function() {
        if (request.status == 200) {
          var response = JSON.parse(request.responseText);
          peer.setRemoteDescription(new RTCSessionDescription(response.answer)).then(function() {
            var candidate = new RTCIceCandidate(response.candidate);
            peer.addIceCandidate(candidate).then(function() {
              console.log("add ice candidate success");
            }).catch(function(err) {
              console.log("error during 'addIceCandidate':", err);
            });
          }).catch(function(e) {
            console.log("error during 'setRemoteDescription':", e);
          });
        }
      };
      request.send(peer.localDescription.sdp);
    }).catch(function(reason) {
      console.log("error during 'createOffer':", reason);
    });
  }
}
