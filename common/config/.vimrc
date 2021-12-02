set nocompatible              " be iMproved, required

set viminfo+=n~/.vim/viminfo " Move the vim info file to a more sensable location

set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()
Plugin 'VundleVim/Vundle.vim'
Plugin 'Valloric/YouCompleteMe'
Plugin 'tpope/vim-fugitive' " Nice Git commands
Plugin 'psliwka/vim-smoothie' " Smooth scrolling with <C-D> and <C-U>
Plugin 'tpope/vim-surround' " Change symbols that surrond text
Plugin 'tpope/vim-commentary' " Comment out a line
Plugin 'git://git.wincent.com/command-t.git'
Plugin 'scrooloose/nerdtree' " For directory listing on side
Plugin 'Quramy/tsuquyomi' " A typescript server for auto complete and definiton search
call vundle#end()

let vim_markdown_preview_github=1

"easier window navigation
nmap <C-h> <C-w>h
nmap <C-j> <C-w>j
nmap <C-k> <C-w>k
nmap <C-l> <C-w>l
nmap <C-z> u
:map <C-n> :NERDTree <Enter>
:map <C-space> :TsuQuickFix <Enter>
imap jj <ESC>

noremap <C-y> "+y
noremap <C-p> "+p

syntax on
filetype plugin indent on
filetype indent on

" NerdTreeDefault
let NERDTreeShowHidden=1


set spelllang=en
set nospell
set linebreak
set tabstop=2
set number relativenumber
set expandtab
set shiftround
set softtabstop=2
set shiftwidth=2
set autoindent
set smarttab
set tags=tags
set completeopt-=preview

let &t_ti.="\<Esc>[1 q"
let &t_SI.="\<Esc>[5 q"
let &t_EI.="\<Esc>[1 q"
let &t_te.="\<Esc>[0 q"

" To allow copying from vim to clipboard
set mouse=v

" You Complete Me config file
let g:ycm_global_ycm_extra_conf = "~/.vim/.ycm_extra_conf.py"