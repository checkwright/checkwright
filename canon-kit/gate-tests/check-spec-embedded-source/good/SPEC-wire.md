# SPEC amendment: wire

The **wire-kind exemption**, exercised here because no other case reaches it: an
amendment stating a wire delta must be able to quote the wire, so a fence of the
configured wire kind is exempt inside an amendment and only inside one. This
block is a verbatim copy of the tracked `widget.proto` and does not fire.

```proto
syntax = "proto3";
package widget.v1;

message WidgetHeader {
  string id = 1;
  string kind = 2;
  int32 count = 3;
  bool enabled = 4;
  string note = 5;
  repeated string tags = 6;
  int64 created_unix = 7;
  string owner = 8;
}
```
