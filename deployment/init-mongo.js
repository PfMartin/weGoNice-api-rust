db.createUser({
  user: 'NiceUser',
  pwd: 'nicePassword',
  roles: [
    {
      role: 'readWrite',
      db: 'weGoNice',
    },
  ],
});
