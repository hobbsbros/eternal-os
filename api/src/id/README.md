# eternalOS Remote ID

A simple library for creating Remote ID data for transmission over 2.4 GHz radio frequency.

# Motivation for Remote ID

The Phoenix quadcopter uses Remote ID in an effort to comply with the U.S. Federal Aviation Administration's new Remote ID requirement.  Beginning September 16, 2023, every unmanned aerial system (UAS) will be required to broadcast location and identification information, enabling law enforcement and air traffic control to more easily detect and monitor unsafe operation of UASes.

More information about Remote ID can be found [on the FAA website](https://www.faa.gov/uas/getting_started/remote_id/).

# Elements of Remote ID Broadcasts

The Phoenix quadcopter seeks to satisfy the *Standard Remote ID* requirement of the Remote ID rule.

The required elements of a Remote ID broadcast include:
- Identity of unmanned aircraft (serial number assigned to the UAS)
- Latitude and longitude of aircraft control station
- Geometric altitude of aircraft control station
- Latitude and longitude of the unmanned aircraft
- Geometric altitude of the unmanned aircraft
- Velocity of the unmanned aircraft
- Timestamp (in UTC) identifying the time of broadcast
- Emergency status of the unmanned aircraft

This and more information can also be found in [14 CFR § 89.305](https://www.ecfr.gov/current/title-14/chapter-I/subchapter-F/part-89/subpart-D/section-89.305).

# Broadcast Specification

The Phoenix quadcopter uses a simple, independently developed, open-source (non-proprietary) broadcast standard called the Phoenix Broadcasting Specification.  The Phoenix Broadcasting Specification is explained below and in `phoenix-spec.pdf` (to be added soon).

## Message Elements

More information coming soon.

## Error Correction

More information coming soon.

# Performance Requirements of Systems with Standard Remote ID

The Phoenix quadcopter is also required by 14 CFR to satisfy minimum performance requirements.

These performance requirements are listed below.

## Self-Testing and Monitoring

Prior to takeoff, the quadcopter will automatically test Remote ID functionality and notify the pilot of the result of the test.  If the Remote ID test fails, the quadcopter will not be able to take off.

The quadcopter will continuously monifor its Remote ID functionality and provide notification of any malfunction or failure to the pilot.

## Tamper Resistance

The quadcopter will be designed in such a way as to limit the ability of an individual to tamper with the Remote ID subsystem.

## Error Correction

The Remote ID subsystem will include error correction in the broadcast of its message elements.

## Interference Considerations

The Remote ID subsystem will not interfere with other systems on the quadcopter, and other systems on the quadcopter will not interfere with the Remote ID functionality.

## Message Broadcast

The quadcopter will broadcast Remote ID signals using a non-proprietary broadcast specification and using a radio frequency compatible with personal wireless devices as specified in [47 CFR part 15](https://www.ecfr.gov/current/title-47/part-15).

## Performance Requirements

The reported geometric positions of both the UAS and the control station will be accurate to 100 feet with 95% probability.

The reported geometric altitude of the UAS will be accurate to 150 feet with 95% probability.

The reported geometric altitude of the control station will be accurate to 15 feet with 95% probability.

The UAS must broadcast its position information no later than 1 second from the time of measurement.

The UAS must broadcast Remote ID at a rate of at least 1 Hz.

## Reference

This and more information can also be found in [14 CFR § 89.310](https://www.ecfr.gov/current/title-14/chapter-I/subchapter-F/part-89/subpart-D/section-89.310).