let data_dir = has('nvim') ? stdpath('data') . '/site' : '~/.vim'
if empty(glob(data_dir . '/autoload/plug.vim'))
  silent execute '!curl -fLo '.data_dir.'/autoload/plug.vim --create-dirs  https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'
  autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif

" Run PlugInstall if there are missing plugins
autocmd VimEnter * if len(filter(values(g:plugs), '!isdirectory(v:val.dir)'))
  \| PlugInstall --sync | source $MYVIMRC
\| endif

call plug#begin()
  Plug 'VundleVim/Vundle.vim'
  Plug 'scrooloose/nerdtree' " For directory listing on side
  Plug 'tpope/vim-fugitive' " Nice Git commands
  Plug 'tpope/vim-surround' " Change symbols that surrond text
  Plug 'psliwka/vim-smoothie' " Smooth vim scrolling
  Plug 'tpope/vim-commentary' " Comment out a line
  Plug 'junegunn/fzf' " Fuzzy find files
  Plug 'junegunn/fzf.vim' " Fuzzy find files
  Plug 'github/copilot.vim' " AI coder
  Plug 'neoclide/coc.nvim', {'branch': 'release'}
  Plug 'dracula/vim', { 'as': 'dracula' }
  Plug 'vim-airline/vim-airline' " Status bar
  Plug 'tpope/vim-fugitive' " Git support
  Plug 'pangloss/vim-javascript'  " Javascript support
  Plug 'leafgarland/typescript-vim' " Typescript syntax highlighting
  " Plug 'sonph/onehalf', { 'rtp': 'vim' } " Nice theme
  Plug 'joshdick/onedark.vim'
  Plug 'peitalin/vim-jsx-typescript' " JSX support
  Plug 'lervag/vimtex' " Latex support
  Plug 'mhinz/vim-startify' " Start screen
call plug#end()

let NERDTreeShowHidden=1 " NerdTreeDefault

" Nice Cursor Tyler
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
set tabstop=4
set number relativenumber
set expandtab
set shiftround
set softtabstop=4
set shiftwidth=4
set autoindent
set smarttab
set tags=tags
set completeopt-=preview
set mouse=v " To allow copying from vim to clipboard
set backspace=indent,eol,start
set updatetime=1000 " Update interval for CursorHold plus a bunch of other thing
set hidden " Allow buffers to be hidden


" ========= Style =========

" Theme
set t_Co=256
set cursorline

colorscheme onedark
let g:airline_theme='onedark'

" 'True' colors
if exists('+termguicolors')
  let &t_8f = "\<Esc>[38;2;%lu;%lu;%lum"
  let &t_8b = "\<Esc>[48;2;%lu;%lu;%lum"
  set termguicolors
endif

" Nice Cursor
let &t_ti.="\<Esc>[1 q"
let &t_SI.="\<Esc>[5 q"
let &t_EI.="\<Esc>[1 q"
let &t_te.="\<Esc>[0 q"

" Airline feature
" Enable the list of buffers
let g:airline#extensions#tabline#enabled = 1
" Show just the filename
let g:airline#extensions#tabline#fnamemod = ':t'

" =========== Auto commands ===============
autocmd BufWritePost *.Xresources  !command xrdb <afile>
autocmd BufWritePost ~/help.md  !command pandoc -s <afile> -o ~/docs/help.pdf

" ============ Navigation ===============
nmap <C-h> <C-w>h
nmap <C-j> <C-w>j
nmap <C-k> <C-w>k
nmap <C-l> <C-w>l

" Easy Escape
imap jj <ESC>

" Buffers
nmap <leader>T :enew<cr> " To open a new empty buffer
nmap <leader>l :bnext<CR> " Move to the next buffer
nmap <leader>h :bprevious<CR> " Move to the previous buffer
nmap gh :bprevious<CR> " Move to the previous buffer
nmap gl :bnext<CR> " Move to the next buffer
nmap <leader>bq :bp <BAR> bd #<CR> " Close the current buffer and move to the previous one

" Easy system copy paste
noremap <A-c> "+y 
noremap <A-v> "+p
imenu disable Help 

" Javascript folding
set foldmethod=syntax "syntax highlighting items specify folds  
" set foldcolumn=1 "defines 1 col at window left, to indicate folding  
let javaScript_fold=1 "activate folding by JS syntax  
set foldlevelstart=99 "start file with all folds opened

" Nerd tree 
nnoremap <C-n> :NERDTree<CR>
" Find current file in nerdtree
nmap ,n :NERDTreeFind<CR>

" fzf search all files names not in gitignore
nnoremap <C-p> :GFiles --cached --others --exclude-standard<CR>
" fzf search all files content
nnoremap <C-f> :Rg<CR>
" fzf search all open buffers
nnoremap <C-b> :Buffers <CR>

" Start screen
let g:startify_bookmarks = [ {'c': '~/.vim/vimrc'}, { 'n': '~/notes'}, {'o': '~/owl'}, '~/p/shirtgen/shirtgen-web' ]

" ========= Language Server ===============

let g:coc_global_extensions = [
  \ 'coc-tsserver',
  \ 'coc-eslint',
  \ 'coc-tailwindcss',
  \ 'coc-prettier'
  \ ]

" Use <c-space> to trigger completion.
if has('nvim')
  inoremap <silent><expr> <c-space> coc#refresh()
else
  inoremap <silent><expr> <c-@> coc#refresh()
endif

" Enter to accept coc option
inoremap <expr> <cr> coc#pum#visible() ? coc#pum#confirm() : "\<CR>"

" coc goto code navigation
nmap <silent> gd <Plug>(coc-definition)
nmap <silent> gD :call CocAction('jumpDefinition', 'vsplit')<CR>
nmap <silent> gy <Plug>(coc-type-definition)
nmap <silent> gi <Plug>(coc-implementation)
nmap <silent> gr <Plug>(coc-references)

" Use `[g` and `]g` to navigate diagnostics
" Use `:CocDiagnostics` to get all diagnostics of current buffer in location list.
nmap <silent> [g <Plug>(coc-diagnostic-prev)
nmap <silent> ]g <Plug>(coc-diagnostic-next)

" Rename symbol under cursor.
nmap <leader>rn <Plug>(coc-rename)

" Applying codeAction to the selected region.
" Example: `<leader>aw` for current word
xmap <leader>a  <Plug>(coc-codeaction-selected)
nmap <leader>a  <Plug>(coc-codeaction-selected)
" Run the Code Lens action on the current line.
nmap <leader>cl  <Plug>(coc-codelens-action)

" Apply AutoFix to problem on the current line.
nmap <leader>qf  <Plug>(coc-fix-current)

function! ShowDocIfNoDiagnostic(timer_id)
  if (coc#float#has_float() == 0 && CocHasProvider('hover') == 1)
    silent call CocActionAsync('doHover')
  endif
endfunction

function! s:show_hover_doc()
  call timer_start(500, 'ShowDocIfNoDiagnostic')
endfunction

autocmd CursorHoldI * :call <SID>show_hover_doc()
autocmd CursorHold * :call <SID>show_hover_doc()

" Highlight the symbol and its references when holding the cursor.
autocmd CursorHold * silent call CocActionAsync('highlight')

" Use K to show documentation in preview window.
nnoremap <silent> K :call ShowDocumentation()<CR>

function! ShowDocumentation()
  if CocAction('hasProvider', 'hover')
    call CocActionAsync('doHover')
  else
    call feedkeys('K', 'in')
  endif
endfunction

" Latex config
let g:vimtex_compiler_latexmk = { 'build_dir' : 'out' }



