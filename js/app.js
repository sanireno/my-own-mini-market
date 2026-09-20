const apiBase = 'http://127.0.0.1:3000';
const cartKey = 'market-cart';
const tokenKey = 'market-token';
const nameKey = 'market-name';
const emailKey = 'market-email';
const getToken = () => localStorage.getItem(tokenKey);
const isAuthenticated = () => Boolean(getToken());
const getCart = () => JSON.parse(localStorage.getItem(cartKey) || '[]');
const saveCart = (cart) => { localStorage.setItem(cartKey, JSON.stringify(cart)); updateCartCount(); };
async function apiRequest(path, options = {}) {
  const headers = { 'Content-Type': 'application/json', ...(options.headers || {}) };
  if (getToken()) headers.Authorization = `Bearer ${getToken()}`;
  let response;
  try {
    response = await fetch(`${apiBase}${path}`, { ...options, headers });
  } catch (error) {
    throw new Error(`Нет соединения с API (${apiBase || window.location.origin}). Запустите cargo run. Детали: ${error.message}`);
  }
  const responseText = await response.text();
  let body = null;
  try {
    body = responseText ? JSON.parse(responseText) : null;
  } catch {
    body = null;
  }
  if (!response.ok) {
    if (response.status === 401) {
      localStorage.removeItem(tokenKey);
      localStorage.removeItem(nameKey);
      localStorage.removeItem(emailKey);
    }
    const serverMessage = body?.error || responseText || 'пустой ответ сервера';
    throw new Error(`API ${response.status} ${response.statusText}: ${serverMessage}`);
  }
  return body;
}
const formatPrice = (value) => `${Number(value).toLocaleString('ru-RU')} ₽`;
function updateCartCount() {
  const count = getCart().reduce((sum, item) => sum + item.quantity, 0);
  document.querySelectorAll('.cart-count').forEach((element) => { element.textContent = count; });
}
async function syncCartCount() {
  if (!isAuthenticated()) return;
  try {
    const cart = await apiRequest('/cart');
    const count = cart.items.reduce((sum, item) => sum + item.quantity, 0);
    document.querySelectorAll('.cart-count').forEach((element) => { element.textContent = count; });
  } catch (error) {
    console.warn('Не удалось синхронизировать счётчик корзины:', error.message);
  }
}
function initAccountMenu() {
  const actions = document.querySelector('.header__actions');
  if (!actions) return;
  const accountLinks = actions.querySelectorAll('.header-account-link');
  accountLinks.forEach((link) => { link.hidden = isAuthenticated(); });
  if (!isAuthenticated()) return;
  const account = document.createElement('div');
  account.className = 'account-menu';
  const username = localStorage.getItem(nameKey) || localStorage.getItem(emailKey) || 'Аккаунт';
  const email = localStorage.getItem(emailKey) || '';
  account.innerHTML = `<button class="account-menu__trigger" type="button" aria-expanded="false"><span class="account-menu__avatar">${username.charAt(0).toUpperCase()}</span><span class="account-menu__name">${username}</span><span class="account-menu__chevron" aria-hidden="true">⌄</span></button><div class="account-menu__panel"><span class="account-menu__label">Вы вошли как</span><strong class="account-menu__username">${username}</strong><span class="account-menu__email">${email}</span><button class="account-menu__logout" type="button">Выйти из аккаунта</button></div>`;
  actions.insertBefore(account, actions.querySelector('.cart-link'));
  apiRequest('/auth/me').then((user) => {
    localStorage.setItem(nameKey, user.username);
    localStorage.setItem(emailKey, user.email);
    account.querySelector('.account-menu__avatar').textContent = user.username.charAt(0).toUpperCase();
    account.querySelector('.account-menu__name').textContent = user.username;
    account.querySelector('.account-menu__username').textContent = user.username;
    account.querySelector('.account-menu__email').textContent = user.email;
  }).catch((error) => console.warn('Не удалось загрузить данные пользователя:', error.message));
  const trigger = account.querySelector('.account-menu__trigger');
  trigger.addEventListener('click', () => {
    const isOpen = account.classList.toggle('is-open');
    trigger.setAttribute('aria-expanded', String(isOpen));
  });
  account.querySelector('.account-menu__logout').addEventListener('click', () => {
    localStorage.removeItem(tokenKey);
    localStorage.removeItem(nameKey);
    localStorage.removeItem(emailKey);
    window.location.href = window.location.pathname.includes('/html/') ? '../index.html' : 'index.html';
  });
}
function addToCart(name, price, image, quantity = 1) {
  const cart = getCart();
  const existing = cart.find((item) => item.name === name);
  if (existing) existing.quantity += quantity;
  else cart.push({ name, price: Number(price), image: image || 'product-card__image--one', quantity });
  saveCart(cart);
}
async function loadProducts() {
  const grid = document.querySelector('.catalog__grid');
  if (!grid) return;
  try {
    const products = await apiRequest('/products?limit=100');
    if (!products.length) return;
    const categoryByProductName = {
      'Лампа Halo': ['home', 'Дом'],
      'Рюкзак Daily': ['work', 'Работа'],
      'Керамическая ваза': ['style', 'Стиль'],
    };
    grid.innerHTML = products.map((product) => {
      const [category, categoryLabel] = categoryByProductName[product.name] || ['all', 'Товар'];
      const imageClass = category === 'work' ? 'product-card__image--two' : category === 'style' ? 'product-card__image--three' : 'product-card__image--one';
      return `<article class="product-card" data-category="${category}"><div class="product-card__image ${imageClass}"></div><div class="product-card__body"><span class="product-card__tag">${categoryLabel}</span><h3>${product.name}</h3><p>${product.description}</p><div class="product-card__footer"><strong>${formatPrice(product.price)}</strong><button class="add-to-cart" data-product-id="${product.id}" data-name="${product.name}" data-price="${product.price}">В корзину +</button></div></div></article>`;
    }).join('');
    bindProductButtons();
  } catch (error) {
    console.warn('Не удалось загрузить каталог:', error.message);
  }
}
function bindProductButtons() {
  document.querySelectorAll('.add-to-cart').forEach((button) => button.addEventListener('click', async () => {
    try {
      if (!isAuthenticated()) {
        addToCart(button.dataset.name, button.dataset.price, button.dataset.image);
      } else if (button.dataset.productId) {
        await apiRequest('/cart/items', { method: 'POST', body: JSON.stringify({ product_id: Number(button.dataset.productId), quantity: 1 }) });
        const cart = await apiRequest('/cart');
        document.querySelectorAll('.cart-count').forEach((element) => { element.textContent = cart.items.reduce((sum, item) => sum + item.quantity, 0); });
      } else {
        throw new Error('Этот товар ещё не добавлен в каталог базы данных');
      }
      button.classList.add('is-added');
      button.textContent = 'Добавлено ✓';
    } catch (error) {
      button.textContent = error.message;
    }
  }));
}
async function renderCart() {
  const list = document.querySelector('[data-cart-list]');
  if (!list) return;
  if (!isAuthenticated()) {
    list.classList.add('cart-locked');
    list.innerHTML = '<div class="cart-lock"><span class="cart-lock__icon">◌</span><h2>Войдите, чтобы открыть корзину</h2><p>Авторизуйтесь, чтобы увидеть товары и оформить заказ.</p><a class="btn btn--primary" href="login.html">Перейти ко входу</a></div>';
    document.querySelectorAll('[data-cart-total]').forEach((element) => { element.textContent = '—'; });
    return;
  }
  try {
    const cart = await apiRequest('/cart');
    list.classList.remove('cart-locked');
    const subtotal = Number(cart.total || 0);
    const shipping = subtotal === 0 ? 0 : subtotal >= 5000 ? 0 : 490;
    list.innerHTML = cart.items.length ? cart.items.map((item) => `<article class="cart-product"><div class="cart-product__image product-card__image--one"></div><div class="cart-product__info"><h3>${item.name}</h3><p>${formatPrice(item.price)} за штуку</p><div class="cart-product__controls"><div class="quantity-control"><button type="button" data-cart-action="decrease" data-id="${item.id}" data-quantity="${item.quantity}" aria-label="Уменьшить количество">−</button><strong>${item.quantity}</strong><button type="button" data-cart-action="increase" data-id="${item.id}" data-quantity="${item.quantity}" aria-label="Увеличить количество">+</button></div><button class="remove-product" type="button" data-cart-action="remove" data-id="${item.id}">Удалить</button></div></div><strong class="cart-product__price">${formatPrice(item.line_total)}</strong></article>`).join('') : '<p class="cart-empty">В корзине пока пусто. Самое время выбрать что-нибудь для дома.</p>';
    document.querySelectorAll('[data-cart-subtotal]').forEach((element) => { element.textContent = formatPrice(subtotal); });
    document.querySelectorAll('[data-cart-shipping]').forEach((element) => { element.textContent = shipping === 0 ? (subtotal ? 'Бесплатно' : '—') : formatPrice(shipping); });
    document.querySelectorAll('[data-cart-total]').forEach((element) => { element.textContent = formatPrice(subtotal + shipping); });
    document.querySelectorAll('.cart-count').forEach((element) => { element.textContent = cart.items.reduce((sum, item) => sum + item.quantity, 0); });
  } catch (error) {
    list.innerHTML = `<p class="cart-empty">${error.message}</p>`;
  }
}
function init() {
  initAccountMenu();
  updateCartCount();
  syncCartCount();
  document.querySelectorAll('.cart-link').forEach((link) => link.addEventListener('click', (event) => {
    if (!isAuthenticated()) {
      event.preventDefault();
      window.location.href = link.getAttribute('href').replace('cart.html', 'login.html');
    }
  }));
  bindProductButtons();
  document.querySelector('[data-cart-list]')?.addEventListener('click', (event) => {
    const button = event.target.closest('[data-cart-action]');
    if (!button) return;
    const quantity = Number(button.dataset.quantity);
    const nextQuantity = button.dataset.cartAction === 'increase' ? quantity + 1 : quantity - 1;
    const request = button.dataset.cartAction === 'remove' || nextQuantity < 1
      ? apiRequest(`/cart/items/${button.dataset.id}`, { method: 'DELETE' })
      : apiRequest(`/cart/items/${button.dataset.id}`, { method: 'PATCH', body: JSON.stringify({ quantity: nextQuantity }) });
    request.then(renderCart).catch((error) => { button.textContent = error.message; });
  });
  document.querySelectorAll('[data-filter]').forEach((button) => button.addEventListener('click', () => {
    document.querySelectorAll('[data-filter]').forEach((tab) => tab.classList.remove('is-active'));
    button.classList.add('is-active');
    document.querySelectorAll('[data-category]').forEach((card) => { card.classList.toggle('is-hidden', button.dataset.filter !== 'all' && card.dataset.category !== button.dataset.filter); });
  }));
  renderCart();
  loadProducts();
  document.querySelectorAll('[data-demo-form]').forEach((form) => form.addEventListener('submit', async (event) => {
    event.preventDefault();
    const note = form.querySelector('.form-note');
    try {
      const isLogin = document.title.includes('Вход');
      const payload = isLogin
        ? { email: form.querySelector('#email').value, password: form.querySelector('#password').value }
        : { username: form.querySelector('#name').value, email: form.querySelector('#email').value, password: form.querySelector('#password').value };
      if (isLogin) {
        const response = await apiRequest('/auth/login', { method: 'POST', body: JSON.stringify(payload) });
        localStorage.setItem(tokenKey, response.token);
        localStorage.setItem(nameKey, response.username || localStorage.getItem(nameKey) || payload.email);
        localStorage.setItem(emailKey, response.email || payload.email);
        window.location.href = 'cart.html';
      } else {
        await apiRequest('/auth/register', { method: 'POST', body: JSON.stringify(payload) });
        localStorage.setItem(nameKey, payload.username);
        localStorage.setItem(emailKey, payload.email);
        note.textContent = 'Аккаунт создан. Теперь войдите.';
        window.setTimeout(() => { window.location.href = 'login.html'; }, 800);
      }
    } catch (error) {
      note.textContent = error.message;
    }
  }));
}
document.addEventListener('DOMContentLoaded', init);
