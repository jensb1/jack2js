var addon = require('../native');
var assert = require('chai').assert;
const { Jack2 } = addon;

describe('Jack2', () => {

  it('Should create ok', () => {
  	assert.isOk(new Jack2());
  });
  it('Should list ports ok', () => {
  	let jack2 = new Jack2()
  	assert.equal(jack2.get_ports()[0], 'system:capture_1');
  });

});