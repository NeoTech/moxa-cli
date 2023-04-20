const restify = require('restify');
const plugins = require('restify').plugins;
const { spawn } = require('child_process');
const fs = require('fs');

const server = restify.createServer();

server.use(plugins.bodyParser());
server.use(plugins.queryParser());


server.get('/*',
  restify.plugins.serveStaticFiles(`${__dirname}/public`)
);

server.post('/upload', (req, res, next) => {
  const file = req.files;
  console.log(file);
  const tempPath = file.upload.path;
  const targetPath = `${__dirname}/uploads/${file.upload.name}`;

  console.log('TmpFile: %s', tempPath);
  console.log('TargetFile: %s', targetPath);

  fs.copyFile(tempPath, targetPath, (err) => {
    if (err) throw err;
    res.send(200, 'File uploaded successfully');
    next();
  });
});

server.get('/list', (req, res, next) => {
  fs.readdir(`${__dirname}/uploads`, (err, files) => {
    if (err) throw err;
    res.json(files);
    next();
  });
});

server.del('/uploads/:filename', function(req, res, next) {
  var filename = req.params.filename;
  var filepath = __dirname + '/uploads/' + filename;
  
  // Check if file exists before deleting
  fs.stat(filepath, function(err, stats) {
    if (err) {
      if (err.code === 'ENOENT') {
        console.log("file does not exist");
        res.send(404, 'File not found');
        next();
      } else {
        console.log(err);
        res.send(500, err);
        next();
      }
    } else {
      console.log("file exists, deleting now");
      fs.unlink(filepath, function(err) {
        if (err) {
          console.log(err);
          res.send(500, err);
          next();
        } else {
          console.log('file deleted successfully');
          res.send(200, 'File deleted');
          next();
        }
      });
    }
  });
});

server.get('/send', (req, res, next) => {
  const fileName = req.query.file;
  const ipAddress = req.query.ip;
  const size = req.query.size;
  const port = req.query.port;
  const interval = req.query.interval;
  const command = './moxa-cli';
  const args = ['-f', __dirname + '/uploads/' + fileName,'-n',ipAddress,'-p',port,'-P',size,'-i',interval];
  const child = spawn(command, args);
  // Handle child process events
  child.stdout.on('data', (data) => {
    console.log(`stdout: ${data}`);
  });
  child.stderr.on('data', (data) => {
    console.error(`stderr: ${data}`);
  });
  child.on('close', (code) => {
    console.log(`child process exited with code ${code}`);
  });
  res.send(200, 'File sent successfully');
  next();
});

server.listen(8080, () => {
  console.log('%s listening at %s', server.name, server.url);
});
