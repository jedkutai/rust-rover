# Raspberry Pi Rover Progress Log

## Phase 1: Initial Setup and Motor Driver Debugging

I did not do a great job documenting the first few steps of the project, so here is a quick recap.

First, I set up the Raspberry Pi so I could SSH into it from my laptop/PC. My goal was to use the Pi headless, without needing a monitor, keyboard, or mouse directly connected to it.

After that, I ran a simple LED test to make sure the Raspberry Pi GPIO pins were working correctly. Once that worked, I connected two motors to a motor driver and a mini breadboard, then connected everything directly to the Pi. The motors worked, I was happy, and I went to sleep.

Later, while watching a YouTube tutorial, I noticed that the person had connected all of the Raspberry Pi GPIO pins to a breadboard using an extension cable. I decided to buy a similar kit to make the wiring cleaner and easier to manage.

When I tried to set up the motors again using the new kit, they did not work. I spent hours rewiring everything and troubleshooting the circuit. Eventually, I found the issue: the pins on the motor driver were not all making proper contact because I had not soldered them, and they were not positioned correctly.

After fixing the connection issue, the motors worked again.

We were so back.

---

## Phase 2: Speed Control

**Objective:** Update the rover so the motors can run at specific speeds instead of only being fully on or fully off.

### 6/26 — 05:00

Sending a specific speed value can result in an error, so I started going through the code and updating the functions I had already created.

This was a bit annoying because I wish I had designed the functions with speed control in mind from the beginning, but it was a good learning experience. We live and learn.

### 6/26 — 06:30

Finished making the changes and confirmed that everything worked.

The motors can now move at specific speeds between `0.0` and `1.0`.

I am considering changing the logic for how steering left and right works. Currently, to turn left, only the right wheels move, and to turn right, only the left wheels move. This works as a tank-style turn, but I am concerned that the inactive wheels may drag against the ground.

I may change the logic so the slower side still moves at 50% speed instead of stopping completely.

Stay tuned.

### 6/26 — 07:45

Stay tuned update: I did it.

The steering logic now allows the opposite side wheels to move at 50% speed while turning instead of stopping completely. I also refactored the code and it looks cute and demure (idk I heard someone say that once).

---

## Next Step: Keyboard Control

The next goal is to control the rover using the keyboard.

Planned controls:

| Key | Action           |
| --- | ---------------- |
| `W` | Move forward     |
| `A` | Steer left       |
| `S` | Move backward    |
| `D` | Steer right      |
| `Q` | Tank turn left   |
| `E` | Tank turn right  |
| `M` | Increase speed   |
| `N` | Decrease speed   |
| `B` | Brake            |
| `X` | Exit the program |

This will make testing the rover much easier because I will be able to control it interactively instead of hardcoding movement sequences.

### 6/26 — 10:45
Guess what? It works. I can control the rover from the keyboard and it drives exactly how I want it.

Next step is to figure out how to wire the sensors. 

I am a bit worried about space and might end up attaching only one of the sensors. We'll see.

### 6/30 — 10:00
Took a long break. Was busy with work and other stuff, but we are back.
I wish I color coded the wires. Todays objective is to figure out the wiring for the four sensors.
Should just be copy paste for all four (I hope).
I am thinking of putting each one on its own mini breadboard.

### 6/30 — 11:00
Yeah still haven't gotten it to work. I am awful at wiring.

Dude. I didn't have the board plugged in this whole time. Jeez man. I stink lol.

### 6/30 — 11:40
Okay. Sensors are all wired. One was connected and tested. Going to do the others after this meeting.
If they all work once connected. I need to figure out how to sit them all on the rover. After that I will worry about the code.


### 7/1 — 02:30
All the sensors are wired and tested. All I need to do is implement the accident detection logic and fit the Pi and batteries onto the rig.
I was originally going to allow the detection to toggle on and off but I am not going to do that anymore.


### 7/1 — 18:30
The half motor for turning idea is cool on paper, but it is horrible in reality.
I am switching to always tank turning no matter what.
I am also making the minumum speed 20% because the rover is to heavy to move at 10%.

The sensor works but it only works if something is head on. Since the rover is much wider than the sensor, 
it still bumps into things/ doesn't turn all the way. 
I have an idea for fixing it but that will be in rover 2.0

I also need to buy better sensors for rover 2.0, this one still bumps into things sometimes.
I am a big fan of this kind of projects. This has been the most fun and frustrating coding project I have ever done.
The coding portion isn't really too difficult, but wiring this thing was an experience.

It's like a different form of playing with legos.


Rover 2.0 ideas
I need a better chasis. One that actually steers.

### 7/7 — 08:30
I am back. Got busy with work but I'm back.
I moved 3 sensors to the front and I'm leaving one in the back.

The front left sensor is facing diagonally to the right.
The front middle sensor is facint straight forwards.
The front right sensor is facing diagonally to the left.

This is to deal this the sensors not picking things up that are on an angle.
If any of the sensors detect and object in the same direction that it is moving, the rover will stop.

When I am finished changing the sensor logic. I will refactor the rest of my code.
