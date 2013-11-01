# The roaming developer
### Hacking the **WFH**


## What do I mean by **roaming**?
* Roaming in several different levels
  * Across places
  * Across teams
  * Across devices
Note:
Talk about different ideas of roaming.
e.g: Joining a new team, getting a new computer, loosing your hard-drive due to Windows 8 Refresh option...


## Nowaday it's **easier**
* Cheaper machines
* Cheaper power
* The Internet


## Nowaday it's **easier**
* Cheaper machines
* Cheaper power
* ~~The Internet~~ The cloud


# **#1** The team must support the roaming developer


# **ZOMG!** THE INTERNET!


## The internet
* I **love** the internet
* Being able to work from different places
* Talking to people from different parts of the world
Note:
Talk about the first IRC talk with someone from Australia, living in the future


# The internet as an **abstraction**
Note:
The Internet can be used as an abstraction of the fisical presence


## Being **collocated**
* Having face to face communication is the most powerfull form of communication


## Being **collocated**
* Having face to face communication is the most powerfull form of communication
* But you don't need to be everyday present fisically


# **Communication**
# is important
### Even collocated teams can communicate better
Note:
Having a better communication, even collocated helps. Using the abstraction of the fisical presence, helps.


## **Virtual** companies
* Ubuntu
* Github
* 37signals
* Several different open-source projects
  * Vagrant
  * Rails
  * Travis CI


# **Communication**
# is important
## And it is challenging


# **#2** The roaming developer must know its tools
Note:
Lets go through some tools that can help you


## **Group chats** are helpful to everyone
### Get everyone in the same page
* IRC
  * Panela velha é que faz comida boa
  * Well stabilished
  * **Bouncers** - Keeps you always online
* Campfire/HipChat
* kandan (oss)
* **Bots**
Note:
If you go this path, use bots. They provide helpful information and announcements


## IRC Bouncer
* Service that **bounces** the messages back to you
* Keep you connected even when your client disconnects
* Can notify you in several different ways
Note:
My setup for freenode


## **VPNs** are you friends
### And sometimes your enemy
* Make sure that everything is setup correctly beforehand
* And that you can access the internet
  * Some VPN might require a proxy to access the internet
  * Having a Raspberry Pi with you can help
    * Poor man's proxy: SOCKS5 + local ssh session


# **Raspberry** Pi


# **#3** Having some help from the project won't hurt


## Virtual development enviroment
* A VM can be pretty helpful to have your project running
* **Provisioning**
  * Chef
  * Puppet
  * Ansible
  * ~~Reliable bash script~~


# **Baseline**


# **Baseline**
#### (a.k.a. awesomeness)


## **Baseline:** Dev sandbox
* Simple projects can take a matter of minutes to be running

```
git pull <url> project
cd project

baseline init
baseline up nodejs redis
baseline ssh

```
Note:
Example from Romulo. Example from coursera sml.


# Come **chat** with **me**


## ~~Crazy~~ Ideas I am **experimenting** with
* Mesh networks for devices
* Owncloud (self-hosted dropbox)
* Gitlab
* Moving away from Gmail's web interface


# Thank **you**
### @bltavares
