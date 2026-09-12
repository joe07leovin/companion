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
- the realtek wifi chip hangs up when the kernels powersaver mode is on
- the watchdog sees the wifi causing a hang in the kernel and issues a reboot 
- initially i thought the reboot was random, but it happened every 2 mins as watch dog was on every two mins
- the fix was to disable the watch dog, the jetson hanged but did not reboot 
- so the next piece of  work was to ssh into the jetson via cable
- diabled the wifi and the jetson did not reboot 
- issued the fix set pcie_asm = off, rebooted
- started a server, pinged from mac 60 sec and 50 mins it worked
- reinstalled stale packages i worked
- rebooted and sshed into through wifi it worked
## Current Config
- wifi on 5Ghz fallback 2.5 Ghz
- jetson connected to tv via hdmi so 2 users are being identified, need to disconnect the tv
-installed tnux, git, rust, vim, practicing vim
## To DO
