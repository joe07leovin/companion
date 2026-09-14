# HAL007 - Companion System
Learn the bare metals of kernel, Vim so i dont become obsolete like Space Odyssey. Interesting to have named the project after the antongonist, and using the same linux 
## Milestones
Sat 12 Sept 2026:
- managed to identify a wifi issue after much trial and error
- managed to ssh into a headless version of ubuntu on a no name nvme very strange
- learnt few commands along the way 
## Bug Log
Sat 12 Sept 2026
- encountered first kernel level bug (relate dto drivers)
- the realtek rtl88x2ce driver deadlocks when PCIe ASPM L1 engages
- PCIe is the service bus that connects to peripharels (wifichip, nvme ssd etc)
- ethool shows the address of connected peripharels
- ASPM is active power management has three modes l0 (link fully awake), l0s (light doze), l1 (deep sleep)
- wifi card <-> cpu link kept napping into L1, driver has bug which prevents it from waking up from L1 properly, causing deadlock
- good intro into deadlock, fown theline can help me understand WAL better
- the watchdog sees the wifi causing a hang in the kernel and issues a reboot 
- initially i thought the reboot was random, but it happened every 2 mins as watch dog was on every two mins
- the fix was to disable the watch dog, the jetson hanged but did not reboot 
- so the next piece of  work was to ssh into the jetson via cable
- diabled the wifi and the jetson did not reboot 
- issued the fix set pcie_aspm = off, rebooted
- started a server, pinged from mac 60 sec and 5 mins it worked
- reinstalled stale packages i worked
- rebooted and sshed into through wifi it worked
Sun 13th Sept 2026
- getting comfortable with vim and learnt vim commands
- getting comfortable with tmux utiling tmux efficiently 
- gettong comfortable woth bash learnt bash commands
- productive day 
## Current Config
- wifi on 5Ghz fallback 2.4 Ghz
- jetson connected to tv via hdmi so 2 users are being identified, need to disconnect the tv
- installed tmux, git, rust, vim, practicing vim
## To DO
