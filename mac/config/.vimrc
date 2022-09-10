call plug#begin()
  Plug 'preservim/nerdtree' " Side bar listing files
  Plug 'tpope/vim-surround' " Surround text with delimiters
  Plug 'tpope/vim-commentary' " Comment out lines
  Plug 'junegunn/fzf.vim' " Fuzzy finder
  Plug 'junegunn/fzf' " Fuzzy finderj
  Plug 'github/copilot.vim' " AI code completion
  Plug 'vim-airline/vim-airline' " Nice status bar
  Plug 'pangloss/vim-javascript'  " Javascript support
  " Plug 'leafgarland/typescript-vim' " Typescript support
  Plug 'HerringtonDarkholme/yats.vim' " Typescript support
  Plug 'peitalin/vim-jsx-typescript' " Typescript jsx support
  Plug 'neoclide/coc.nvim', {'branch': 'release'} " Code completion
  Plug 'psliwka/vim-smoothie' " Smooth vim scrolling
  Plug 'tpope/vim-fugitive' " Git integration
  Plug 'sonph/onehalf', { 'rtp': 'vim' } " Nice theme
call plug#end()

syntax on 
filetype plugin indent on


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
set re=0
set backspace=indent,eol,start
set updatetime=1000 " Update interval for CursorHold plus a bunch of other thing
set hidden " Allow buffers to be hidden
set foldcolumn=0 " No fold column (the side column) Hiding becausse some plugin put weird things in it

" ========= Style =========

" Theme
set t_Co=256
set cursorline
colorscheme onehalfdark
let g:airline_theme='onehalfdark'

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


" Javascript folding
set foldmethod=syntax "syntax highlighting items specify folds  
set foldcolumn=1 "defines 1 col at window left, to indicate folding  
let javaScript_fold=1 "activate folding by JS syntax  
set foldlevelstart=99 "start file with all folds opened

" Buffers
nmap <leader>T :enew<cr> " To open a new empty buffer
nmap <leader>l :bnext<CR> " Move to the next buffer
nmap <leader>h :bprevious<CR> " Move to the previous buffer
nmap gh :bprevious<CR> " Move to the previous buffer
nmap gl :bnext<CR> " Move to the next buffer
nmap <leader>bq :bp <BAR> bd #<CR> " Close the current buffer and move to the previous one

" Close the current buffer and move to the previous one
command Bq :bp <BAR> bd #<CR> 

" Search
" fzf search all files names not in gitignore
nnoremap <C-p> :GFiles --cached --others --exclude-standard<CR>
" fzf search all files content
nnoremap <C-f> :Rg <CR>
" fzf search all open buffers
nnoremap <C-b> :Buffers <CR>


" Easy Escape
imap jj <Esc>

" Nerd tree 
nnoremap <C-n> :NERDTree<CR>
" Find current file in nerdtree
nmap ,n :NERDTreeFind<CR>



" ========= Language Server ===============

let g:coc_global_extensions = [
  \ 'coc-tsserver'
  \ ]

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

