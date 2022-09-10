call plug#begin()
Plug 'VundleVim/Vundle.vim'
Plug 'scrooloose/nerdtree' " For directory listing on side
Plug 'tpope/vim-fugitive' " Nice Git commands
Plug 'tpope/vim-surround' " Change symbols that surrond text
Plug 'tpope/vim-commentary' " Comment out a line
Plug 'junegunn/fzf' " Fuzzy find files
Plug 'junegunn/fzf.vim' " Fuzzy find files
Plug 'github/copilot.vim' " AI coder
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'dracula/vim', { 'as': 'dracula' }
Plug 'vim-airline/vim-airline' " Status bar
Plug 'tpope/vim-fugitive' " Git support
Plug 'leafgarland/typescript-vim' " Typescript syntax highlighting
call plug#end()

let NERDTreeShowHidden=1 " NerdTreeDefault

" Nice Cursor
let &t_ti.="\<Esc>[1 q"
let &t_SI.="\<Esc>[5 q"
let &t_EI.="\<Esc>[1 q"
let &t_te.="\<Esc>[0 q"

syntax on
filetype plugin indent on
" colorscheme dracula

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
set backspace=indent,eol,start

" Auto commands
autocmd BufWritePost *.Xresources  !command xrdb <afile>

"easier window navigation
nmap <C-h> <C-w>h
nmap <C-j> <C-w>j
nmap <C-k> <C-w>k
nmap <C-l> <C-w>l

" Easy Escape
imap jj <ESC>

" Easy system copy paste
noremap <A-c> "+y 
noremap <A-v> "+p
imenu disable Help 
" Nerd tree 
nnoremap <C-n> :NERDTree<CR>
" Find current file in nerdtree
nmap ,n :NERDTreeFind<CR>

" fzf search all files names not in gitignore
nnoremap <C-p> :GFiles --cached --others --exclude-standard<CR>
" fzf search all files content
nnoremap <C-f> :Rg<CR>

" Enter to accept coc option
inoremap <expr> <cr> coc#pum#visible() ? coc#pum#confirm() : "\<CR>"

" coc goto code navigation
nmap <silent> gd <Plug>(coc-definition)
nmap <silent> gD :call CocAction('jumpDefinition', 'vsplit')<CR>
nmap <silent> gy <Plug>(coc-type-definition)
nmap <silent> gi <Plug>(coc-implementation)
nmap <silent> gr <Plug>(coc-references)

" Run the Code Lens action on the current line.
nmap <leader>cl  <Plug>(coc-codelens-action)

" Tooltips on hover
nnoremap <silent> <leader>h :call CocActionAsync('doHover')<cr>

" Highlight the symbol and its references when holding the cursor.
autocmd CursorHold * silent call CocActionAsync('highlight')

