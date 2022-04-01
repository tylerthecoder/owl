let NERDTreeShowHidden=1 " NerdTreeDefault

"easier window navigation
nmap <C-h> <C-w>h
nmap <C-j> <C-w>j
nmap <C-k> <C-w>k
nmap <C-l> <C-w>l
map <C-n> :NERDTree <Enter>
imap jj <ESC>
" Easy system copy paste
noremap <A-c> "+y 
noremap <A-v> "+p
imenu disable Help 

" Nice Cursor
let &t_ti.="\<Esc>[1 q"
let &t_SI.="\<Esc>[5 q"
let &t_EI.="\<Esc>[1 q"
let &t_te.="\<Esc>[0 q"

syntax on
filetype plugin indent on
colorscheme default

set nocompatible              " be iMproved, required
set viminfo+=n~/.vim/viminfo " Move the vim info file to a more sensable location
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
set mouse=v " To allow copying from vim to clipboard

" Auto commands
autocmd BufWritePost *.Xresources  !command xrdb <afile>
" autocmd InsertLeave *.rs              :YcmForceCompileAndDiagnostics



set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()
Plugin 'VundleVim/Vundle.vim'
Plugin 'Valloric/YouCompleteMe'
Plugin 'scrooloose/nerdtree' " For directory listing on side
Plugin 'tpope/vim-fugitive' " Nice Git commands
Plugin 'tpope/vim-surround' " Change symbols that surrond text
Plugin 'tpope/vim-commentary' " Comment out a line
call vundle#end()

" You Complete Me Config
let g:ycm_min_num_of_chars_for_completion = 1
