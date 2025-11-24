<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# I started getting the following warning when opening up Photoshop on my mac today:

> Your scratch disk is getting full. This could prevent your computer from saving work properly. Free space available: 28 GB Recommended: 100 GB

I haven't gotten that error in literally decades. I'm only using 1.3TB of my system's 2TB of storage and the attached screenshot is from my storage settings.

---

What's the best tool to utilize to get visibility into what could be taking up all of the scratch disk to clean it all up?

Based on your storage screenshot and Photoshop scratch disk warning, I can identify the issue and recommend the best tools to diagnose and resolve this problem.

## Understanding Your Situation

Your Mac shows 350.72 GB of **System Data**, which is abnormally large (normal is around 12 GB). This category includes hidden files that macOS doesn't classify as Documents, Applications, or other standard categories—including cache files, temporary files, Time Machine snapshots, and importantly for your case, Adobe application caches.[^1_1][^1_2]

The Photoshop scratch disk warning with only 28 GB free (despite showing 1.3 TB total usage) suggests that hidden cache files—likely from Adobe applications—are consuming massive amounts of space that aren't visible in standard storage views.

## Best Tools for Disk Space Analysis

After evaluating current options for Mac disk space analysis in 2025, here are the top recommendations:

### **1. DaisyDisk (\$14.99) - Best Overall**

**Why it's ideal for your situation:**

- Beautiful, interactive visual "flower" interface that makes large files immediately obvious[^1_3][^1_4]
- Can detect and visualize **hidden and purgeable space**—critical for finding Adobe caches[^1_5]
- Scans cloud drives (Dropbox, Google Drive) and network storage[^1_6]
- Fast scanning with real-time updates[^1_4]
- Compatible with latest macOS versions including Sequoia and M-series Macs[^1_3]
- One-time purchase (not subscription)[^1_6]

**Best for:** Users who want visual clarity and manual control over what gets deleted[^1_7][^1_6]

### **2. OmniDiskSweeper (FREE) - Best Free Option**

**Why it's excellent:**

- Completely free from The Omni Group[^1_8][^1_3]
- Simple text-based list interface showing files in descending order by size[^1_8][^1_3]
- No-frills but highly effective—gets straight to the point[^1_9]
- Compatible with latest macOS versions[^1_10][^1_8]
- Warns you before deleting critical system files[^1_9]

**Best for:** Users who prefer straightforward lists and don't need fancy visualizations[^1_11]

### **3. GrandPerspective (FREE or \$2.99) - Best Budget Visual Tool**

**Why it's solid:**

- Free from SourceForge or \$2.99 on App Store[^1_12][^1_13]
- Treemap visualization (similar to Windows' WinDirStat)[^1_14][^1_12]
- Fast scanning and responsive interface[^1_12]
- Can filter and search files by name, type, or size[^1_12]
- Extensive documentation[^1_12]

**Best for:** Users wanting visual analysis without spending much[^1_15]

### **4. CleanMyMac X (\$34.95/year) - Most Comprehensive**

**Why it's powerful but pricey:**

- All-in-one solution with automated junk removal, malware scanning, and optimization[^1_5][^1_3]
- Space Lens feature visualizes disk usage[^1_6]
- Automatically identifies and removes Adobe caches, system logs, and other hidden files[^1_5]
- Apple-notarized (verified safe)[^1_5]
- Can reclaim enormous amounts of space automatically[^1_5]

**Best for:** Users wanting automated maintenance and willing to pay subscription fees[^1_7][^1_6]

## Recommended Approach for Your Specific Issue

Given that you're dealing with Adobe/Photoshop cache issues creating the scratch disk warning, here's my recommended strategy:

### **Immediate Solution (Free):**

1. **Download OmniDiskSweeper** (free) to quickly identify large files
2. **Manually clear Adobe caches** using these locations:
    - `~/Library/Application Support/Adobe/Common/Media Cache Files/` (can contain 10GB-500GB+)[^1_16][^1_17]
    - `~/Library/Caches/Adobe/` (check After Effects, Premiere, Photoshop folders)[^1_17][^1_18][^1_16]
    - `/tmp` folder (for Photoshop temp files)[^1_19][^1_20]
3. **Within Photoshop:**
    - Go to **Edit > Purge > All** to clear scratch disks[^1_21][^1_19]
    - Check **Preferences > Scratch Disks** to see actual free space[^1_22]
    - Consider redirecting scratch disk to external SSD if available[^1_19][^1_21]

### **Long-term Solution:**

**Invest in DaisyDisk (\$14.99)** for ongoing storage management. It excels at:

- Visualizing hidden System Data that OmniDiskSweeper might miss[^1_5]
- Showing purgeable space[^1_5]
- Making it easy to spot when Adobe caches balloon again[^1_23]
- One-time purchase vs. CleanMyMac's yearly subscription[^1_6]


## Additional Findings About Your 350GB System Data

Your abnormally large System Data likely contains:[^1_2][^1_24][^1_1]

- **Adobe application caches** (After Effects can alone consume 350-590GB!)[^1_24][^1_18][^1_25]
- **Time Machine local snapshots** (can be hundreds of GB)[^1_26][^1_1]
- **iCloud data stored locally** (if you keep files offline)[^1_24]
- **Old system logs and caches**[^1_1][^1_2]

One user freed **250 GB** by finding Adobe and Dropbox caches in hidden Library folders. Another found **588 GB** just from After Effects caches.[^1_18][^1_23]

## Quick Commands to Reveal Hidden Files

To manually investigate before downloading tools:

1. In Finder, press **Command + Shift + .** (period) to show hidden files[^1_27][^1_28]
2. Navigate to **~/Library/** and check folder sizes (press Command + I for info)[^1_26]
3. Check these specific folders:
    - `~/Library/Caches/Adobe/`
    - `~/Library/Application Support/Adobe/Common/`
    - `~/Library/Containers/` (can contain huge mediaanalysisd caches)[^1_24]

## Final Recommendation

**For your immediate needs:** Use the free **OmniDiskSweeper** combined with manual Adobe cache cleaning (following paths above).

**For long-term convenience:** Invest in **DaisyDisk** (\$14.99 one-time). Its visualization capabilities will help you immediately spot when caches build up again, and the one-time cost makes it more economical than CleanMyMac's subscription model.

Both tools are safe, well-reviewed, and compatible with your Mac setup. DaisyDisk has been around for 20 years and OmniDiskSweeper is from the respected Omni Group, so you're getting proven, reliable software either way.[^1_13][^1_8]
<span style="display:none">[^1_29][^1_30][^1_31][^1_32][^1_33][^1_34][^1_35][^1_36][^1_37][^1_38][^1_39][^1_40][^1_41][^1_42][^1_43][^1_44][^1_45][^1_46][^1_47][^1_48][^1_49][^1_50][^1_51][^1_52][^1_53][^1_54][^1_55][^1_56]</span>

<div align="center">⁂</div>

[^1_1]: https://mackeeper.com/blog/why-is-system-data-so-big-on-mac/

[^1_2]: https://cleanmymac.com/blog/why-system-storage-takes-so-much-space

[^1_3]: https://onmac.net/best-mac-disk-space-analyzers/

[^1_4]: https://www.drbuho.com/how-to/best-disk-space-analyzer-mac

[^1_5]: https://www.techycub.com/mac-cleaner/daisydisk-vs-cleanmymac.html

[^1_6]: https://www.drbuho.com/review/daisydisk-vs-cleanmymac

[^1_7]: https://www.youtube.com/watch?v=t87wOwvAigE

[^1_8]: https://omnidisksweeper.macupdate.com

[^1_9]: https://thesweetbits.com/best-mac-cleaner-software/

[^1_10]: https://slashdot.org/software/p/OmniDiskSweeper/

[^1_11]: https://www.podfeet.com/blog/2025/08/disk-sweeping-apps/

[^1_12]: https://sourceforge.net/projects/grandperspectiv/

[^1_13]: https://grandperspectiv.sourceforge.net

[^1_14]: https://dev.to/kojix2/scan-your-linux-disk-and-visualize-it-on-mac-with-grandperspective-3mn6

[^1_15]: https://www.reddit.com/r/MacOS/comments/1czfdbz/why_dont_people_recommend_disk_inventory_x_and/

[^1_16]: https://www.youtube.com/watch?v=ORQ-WpHa5sI

[^1_17]: https://chrispennington.blog/blog/clear-mac-space-delete-adobe-cache/

[^1_18]: https://discussions.apple.com/thread/255249340

[^1_19]: https://www.youtube.com/watch?v=Vhat9i1EOWI

[^1_20]: https://www.remosoftware.com/info/how-to-find-photoshop-temp-files-on-mac

[^1_21]: https://www.youtube.com/watch?v=zl9hibepUJY

[^1_22]: https://community.adobe.com/t5/photoshop-ecosystem-discussions/why-aren-t-photoshop-temp-files-showing-up-in-tmp-folder/td-p/12674280

[^1_23]: https://www.youtube.com/watch?v=7JRVZJDyyxw

[^1_24]: https://discussions.apple.com/thread/255806791

[^1_25]: https://community.adobe.com/t5/photoshop-ecosystem-discussions/adobe-cache-takes-up-too-much-space-in-my-macbook-s-system-data/m-p/15221315

[^1_26]: https://discussions.apple.com/thread/254588225

[^1_27]: https://blog.greggant.com/posts/2022/04/10/reclaiming-space-from-system-data-in-macos.html

[^1_28]: https://www.reddit.com/r/photoshop/comments/1b9jua1/macos_scratch_disk_exact_file_location/

[^1_29]: CleanShot-2025-11-23-at-16.40.47.jpg

[^1_30]: https://community.adobe.com/t5/photoshop-ecosystem-discussions/p-how-to-fix-photoshop-scratch-disks-full-error/td-p/9610332

[^1_31]: https://apps.apple.com/us/app/disk-space-analyzer-inspector/id446243721

[^1_32]: https://www.reddit.com/r/photoshop/comments/17dr5x7/scratch_disks_full_insanity/

[^1_33]: https://disk-space-analyzer.macupdate.com

[^1_34]: https://helpx.adobe.com/photoshop/desktop/troubleshoot/troubleshoot-tools-resources/set-up-and-manage-scratch-disks.html

[^1_35]: https://www.reddit.com/r/macapps/comments/14x6hbz/mac_disk_usage_analyzer_utility_as_fast_as_wiztree/

[^1_36]: https://www.avg.com/en/signal/how-to-clear-photoshop-scratch-disk-mac

[^1_37]: https://www.facebook.com/groups/petermacdoctor/posts/4023924491251408/

[^1_38]: https://www.reddit.com/r/mac/comments/ynv4d0/system_data_taking_up_all_my_storage_how_do_i_fix/

[^1_39]: https://discussions.apple.com/thread/2383653

[^1_40]: https://daisydiskapp.com

[^1_41]: https://apps.apple.com/us/app/grandperspective/id1111570163

[^1_42]: https://slashdot.org/software/comparison/OmniDiskSweeper-vs-macOS-Catalina/

[^1_43]: https://mackeeper.com/blog/daisydisk-vs-cleanmymac/

[^1_44]: https://www.reddit.com/r/MacOS/comments/1i1w9vx/mac_cleaning_apps_daisydisk_and_cleanmymac_giving/

[^1_45]: https://grandperspective.macupdate.com

[^1_46]: https://cleanmymac.com/blog/daisydisk-review

[^1_47]: https://discussions.apple.com/thread/254575386

[^1_48]: https://www.reddit.com/r/MacOS/comments/fj24k9/daisydisk_vs_disk_doctor_vs_cleanmymac_x/

[^1_49]: https://forums.macrumors.com/threads/why-does-system-data-keep-increasing.2419555/

[^1_50]: https://www.reddit.com/r/Adobe/comments/1o5j8o2/how_do_i_find_old_media_cache_folders_to_delete/

[^1_51]: https://proedu.com/blogs/photography-fundamentals/scratch-disks-are-full-photoshop-mac-quick-solutions-for-memory-issues

[^1_52]: https://www.reddit.com/r/MacOS/comments/154rp99/how_to_do_i_clear_system_data_on_mac_os/

[^1_53]: https://www.skillshare.com/en/blog/how-to-clear-your-scratch-disk-photoshop-101/

[^1_54]: https://talk.macpowerusers.com/t/sequoia-and-full-disks/39719

[^1_55]: https://helpx.adobe.com/premiere/desktop/troubleshooting/media-issues/automatically-manage-your-media-cache-files.html

[^1_56]: https://www.macworld.com/article/676493/how-to-delete-system-storage-on-mac.html


---

# ok, I actually already own CleanMyMac and just ran it and did the first large cleanup task and it appears to have deleted an important dev directory/file. Now when I open up my terminal, it immediately shows this error:

/Users/jordanhudgens/.zshrc:.:355: no such file or directory: /Users/jordanhudgens/.local/bin/env

Additionally, it now appears like rvm is corrupted, it's installed on the system, but our primary ruby version of 3.3.10 says it's no longer installed and if I go to one of our rails applications, it shows this before loading the terminal:

To install do: 'rvm install "ruby-3.3.10"'

But running it raises a number of errors. I believe I was able to get rbenv installed and running properly, so please analyze the attached zshrc file and provide instructions for:

- Switching to rbenv for the ruby manager and so the system uses rbenv by default instead of rvm
- Fully uninstalling rvm since the current implementation is a corrupted version
- Fixing the `/Users/jordanhudgens/.zshrc:.:355: no such file or directory: /Users/jordanhudgens/.local/bin/env` error that shows up on the initial terminal load.

I apologize for the CleanMyMac disaster—unfortunately, it's known for being overly aggressive with developer tools. Let me help you fix all three issues and properly migrate from RVM to rbenv.

## Overview of the Problems

Based on your `.zshrc` file analysis:[^2_1]

1. **Line 355 error**: CleanMyMac deleted `/Users/jordanhudgens/.local/bin/env` (likely part of a tool installation)
2. **Corrupted RVM**: The RVM installation is broken but still referenced in your config
3. **Need to switch to rbenv**: You have rbenv installed but not properly configured

## Step-by-Step Solution

### **Phase 1: Fix the Immediate `.local/bin/env` Error**

This line is at the bottom of your `.zshrc` file (line 355). Let's remove or comment it out:

```bash
# Open your .zshrc in your preferred editor
vim ~/.zshrc

# Or use nano if you prefer
nano ~/.zshrc
```

**Find this line near the end:**

```bash
. "$HOME/.local/bin/env"
```

**Comment it out by adding `#` at the beginning:**

```bash
# . "$HOME/.local/bin/env"
```

Save and close the file. This will stop the immediate error on terminal launch.

***

### **Phase 2: Completely Uninstall RVM**

RVM and rbenv cannot coexist. Here's the complete removal process:[^2_2][^2_3]

```bash
# Step 1: Run RVM's self-destruct command
rvm implode
# When prompted, type 'yes' to confirm

# Step 2: Manually remove any remaining RVM directories
rm -rf ~/.rvm
rm -rf ~/.rvmrc
sudo rm -rf /etc/rvmrc
sudo rm -rf /etc/profile.d/rvm.sh

# Step 3: Remove the RVM gem (if installed as a gem)
gem uninstall rvm
```

**Now edit your `.zshrc` to remove RVM references:**

Find and **remove or comment out** these lines in your `.zshrc`:

```bash
# Line 54 - Remove this:
export PATH="$PATH:$HOME/.rvm/bin"

# Line 133 - Remove 'rvm' from the plugins list
# Change this:
plugins=(
  git
  rvm          # <--- REMOVE THIS LINE
  rails
  autojump
  # ... rest of plugins
)

# To this:
plugins=(
  git
  rails
  autojump
  xcode
  npm
  macos
  web-search
  vscode
  vi-mode
  common-aliases
  dirhistory
  heroku
  nvm
  chucknorris
  zsh-autosuggestions
)
```


***

### **Phase 3: Configure rbenv Properly**

Since you mentioned rbenv is already installed, let's configure it correctly:[^2_4][^2_5][^2_6][^2_2]

**1. Verify rbenv is installed:**

```bash
which rbenv
# Should output: /opt/homebrew/bin/rbenv or similar
```

**2. Edit your `.zshrc` to enable rbenv:**

Find these commented lines (around line 48-50):

```bash
# Uncomment the following lines if you decide to use rbenv
# export PATH="$HOME/.rbenv/bin:$PATH"
# eval "$(rbenv init -)"
```

**Replace them with** (uncommented and properly configured for Homebrew installation):

```bash
# rbenv configuration
export PATH="$HOME/.rbenv/bin:$PATH"
eval "$(rbenv init - zsh)"
```

**IMPORTANT**: Place these lines **AFTER** the Homebrew PATH configuration (after line 12) but **BEFORE** the `source $ZSH/oh-my-zsh.sh` line (line ~140).[^2_7][^2_8][^2_5]

The ideal placement in your `.zshrc` should be:

```bash
####### PATH ENVS ########
export PATH="/opt/homebrew/opt/coreutils/libexec/gnubin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:$PATH"

# ... other PATH exports ...

# rbenv configuration - ADD THIS SECTION HERE
export PATH="$HOME/.rbenv/bin:$PATH"
eval "$(rbenv init - zsh)"

# NVM (Node Version Manager) setup
export NVM_DIR="$HOME/.nvm"
# ... rest of file
```

**3. Reload your shell configuration:**

```bash
source ~/.zshrc
```

**4. Verify rbenv is working:**

```bash
rbenv --version
# Should show: rbenv 1.2.0 or similar

which ruby
# Should show: /Users/jordanhudgens/.rbenv/shims/ruby
```


***

### **Phase 4: Install Ruby 3.3.10 with rbenv**

Now install the Ruby version you were using:[^2_9][^2_10][^2_4]

```bash
# Step 1: List available Ruby versions
rbenv install -l | grep 3.3

# Step 2: Install Ruby 3.3.10
rbenv install 3.3.10

# Step 3: Set it as your global default
rbenv global 3.3.10

# Step 4: Rehash to update shims (critical step!)
rbenv rehash

# Step 5: Verify installation
ruby -v
# Should output: ruby 3.3.10 ...
```

**5. Install bundler for your Rails projects:**

```bash
gem install bundler
rbenv rehash
```

**6. Navigate to your Rails projects and install gems:**

```bash
cd ~/code/dashtrack  # or your project directory
bundle install
rbenv rehash
```


***

## Complete Updated `.zshrc` Configuration Summary

Here are the **key changes** you need to make to your `.zshrc`:

### **Lines to REMOVE:**

1. Line 54: `export PATH="$PATH:$HOME/.rvm/bin"`
2. Line 133: `rvm` from plugins list
3. Line 355 (near end): `. "$HOME/.local/bin/env"`

### **Lines to ADD/UNCOMMENT:**

After your PATH configurations (around line 40), add:

```bash
# rbenv configuration
export PATH="$HOME/.rbenv/bin:$PATH"
eval "$(rbenv init - zsh)"
```


***

## Verification Commands

After making all changes, close and reopen your terminal, then run:

```bash
# Should show no errors
echo "Terminal loaded successfully"

# Should point to rbenv
which ruby
# Expected: /Users/jordanhudgens/.rbenv/shims/ruby

# Should show 3.3.10
ruby -v

# Should work without errors
cd ~/code/dashtrack
bundle exec rails -v
```


***

## Why rbenv Over RVM?

For your reference, rbenv is generally preferred because:[^2_11][^2_2][^2_4]

- **Simpler architecture**: Doesn't override `cd` or `gem` commands
- **Shim-based**: Uses PATH manipulation instead of function overrides
- **Lighter weight**: Less shell startup overhead
- **More predictable**: Doesn't modify your environment as aggressively
- **Better for automated tools**: Works more reliably with CI/CD and deployment scripts

***

## Troubleshooting

**If you still see RVM warnings after restarting:**

```bash
# Check for any remaining RVM references
grep -r "rvm" ~/.zshrc ~/.zshenv ~/.zprofile 2>/dev/null
```

**If `rbenv init` doesn't work:**

```bash
# Ensure rbenv was installed via Homebrew
brew install rbenv ruby-build

# Then re-add the init lines to .zshrc
```

**If gems aren't found after installation:**

```bash
rbenv rehash
```

This command must be run after installing any gem that provides executables.[^2_10][^2_4]

The migration should now be complete, with your terminal loading without errors and rbenv managing Ruby 3.3.10 properly for your Rails applications.[^2_1]
<span style="display:none">[^2_12][^2_13][^2_14][^2_15][^2_16][^2_17][^2_18][^2_19][^2_20][^2_21]</span>

<div align="center">⁂</div>

[^2_1]: zshrc.txt

[^2_2]: https://reinteractive.com/articles/tutorial-series-for-experienced-rails-developers/rvm-to-rbenv-migration

[^2_3]: http://jonathan-jackson.net/rvm-and-rbenv

[^2_4]: https://elliotekj.com/posts/2016/09/29/how-to-switch-from-rvm-to-rbenv

[^2_5]: https://betterstack.com/community/guides/scaling-ruby/rbenv-explained/

[^2_6]: https://programmingzen.com/installing-rbenv-on-zsh-on-macos/

[^2_7]: https://github.com/rbenv/rbenv/issues/1508

[^2_8]: https://github.com/rbenv/rbenv/issues/1572

[^2_9]: https://mac.install.guide/ruby/9

[^2_10]: https://gist.github.com/akdetrick/7604130

[^2_11]: https://dev.to/krtb/why-and-how-i-replaced-rvm-with-rbenv-23ad

[^2_12]: https://installvirtual.com/how-to-uninstall-rvm-implode-rvm-on-mac/

[^2_13]: http://geekhmer.github.io/blog/2015/04/12/how-to-uninstall-rvm

[^2_14]: https://rvm.io/rubies/removing

[^2_15]: https://batsov.com/articles/2025/05/20/switching-from-zsh-to-fish/

[^2_16]: https://stackoverflow.com/questions/3950260/how-to-uninstall-rvm

[^2_17]: https://stackoverflow.com/questions/64634457/cannot-resolve-rbenv-init-instruction-for-zsh-and-oh-my-zsh-on-mac

[^2_18]: https://garyrafferty.com/software/Installing-rbenv-on-osx/

[^2_19]: https://teamtreehouse.com/community/trouble-uninstalling-rvm-in-order-to-install-rbenv

[^2_20]: https://github.com/rbenv/rbenv

[^2_21]: https://www.youtube.com/watch?v=roTArTNov9g
